use std::{env, fs, io, path::Path, vec};

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

    let input = expected_input
      .split(',')
      .map(|i| i.parse().unwrap())
      .collect();

    TestIoHandler {
      input,
      output: String::new(),
      expected_output,
    }
  }
}

impl IoHandler for TestIoHandler {
  type Err = ();

  fn read_input(&mut self) -> Result<CellValue, Self::Err> {
    Ok(self.input.remove(0))
  }

  fn write_output(&mut self, output: &[CellValue]) -> Result<(), Self::Err> {
    for c in output {
      self.output.push(*c as char);
    }

    Ok(())
  }

  fn flush(&mut self) -> Result<(), Self::Err> {
    Ok(())
  }
}

fn scan_dir(name: &Path) -> io::Result<Vec<(String, String)>> {
  let content = fs::read_dir(name)?;

  let mut files = vec![];

  for file in content {
    let file = file?;
    let path = file.path();

    if path.is_dir() {
      files.append(&mut scan_dir(&path)?);
    }

    files.push((
      file.file_name().to_str().unwrap().to_string(),
      fs::read_to_string(&path)?,
    ))
  }

  Ok(files)
}

#[test]
fn run_resources() -> io::Result<()> {
  env::set_var("RUST_LOG", "debug");

  let files = scan_dir(&Path::new("tests/resources").to_path_buf())?;

  for (filename, content) in files {
    if !filename.ends_with(".brain") {
      continue;
    }

    let instructions = parser::parse_file(&content);
    let io_handler = TestIoHandler::build_from_file(content.lines().collect());

    let mut runtime = Runtime::new(instructions, 30, io_handler);

    let result = runtime.run();

    assert!(result.is_ok());

    assert_eq!(
      runtime.io_handler.output, runtime.io_handler.expected_output,
      "\n\n###\nTest failed for file ({})\n###\n\n",
      filename
    );
  }

  Ok(())
}
