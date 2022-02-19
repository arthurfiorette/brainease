use std::{
  env, fs,
  io::Result,
  path::{Path, PathBuf},
  vec,
};

use brainease_lexer::{parser, syntax::CellValue};
use brainease_runtime::{io_handler::IoHandler, runtime::Runtime};

#[derive(Debug, Clone)]
struct TestIoHandler {
  input: Vec<CellValue>,

  output: String,
  expected_output: String,
}

impl TestIoHandler {
  fn build_from_file(lines: Vec<&str>) -> TestIoHandler {
    let expected_output = lines.get(1).unwrap()[1..].trim().to_string();
    let expected_input = lines.get(2).unwrap()[1..].trim();

    let input = if expected_input.is_empty() {
      vec![]
    } else {
      expected_input
        .split(',')
        .map(|s| s.parse::<CellValue>().unwrap())
        .collect()
    };

    TestIoHandler {
      input,
      output: String::new(),
      expected_output,
    }
  }
}

impl IoHandler for TestIoHandler {
  fn read_input(&mut self) -> CellValue {
    self.input.remove(0)
  }

  fn write_output(&mut self, output: CellValue) {
    self.output.push(output as char);
  }
}

fn scan_dir(name: &PathBuf) -> Result<Vec<String>> {
  let content = fs::read_dir(name)?;

  let mut files: Vec<String> = vec![];

  for file in content {
    let file = file?;
    let path = file.path();

    if path.is_dir() {
      files.append(&mut scan_dir(&path)?);
    }

    files.push(fs::read_to_string(&path)?)
  }

  Ok(files)
}

#[test]
fn run_resources() -> Result<()> {
  env::set_var("RUST_LOG", "debug");

  let files = scan_dir(&Path::new("tests/resources").to_path_buf())?;

  for file in files {
    let instructions = parser::parse_file(&file);
    let io_handler = TestIoHandler::build_from_file(file.lines().collect());

    let mut runtime = Runtime::<TestIoHandler>::build(instructions, 30, io_handler);

    runtime.run();

    assert_eq!(
      runtime.io_handler.output,
      runtime.io_handler.expected_output
    );
  }

  Ok(())
}