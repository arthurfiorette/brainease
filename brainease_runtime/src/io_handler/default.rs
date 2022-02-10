use std::io::{stdin, stdout, Read, Write};

use brainease_lexer::syntax::CellValue;

use super::IoHandler;

#[derive(Debug, Clone)]
pub struct DefaultIoHandler {}

impl IoHandler for DefaultIoHandler {
  fn read_input(&mut self) -> CellValue {
    println!("Enter input: ");

    let mut data = [0; 1];
    stdin()
      .read_exact(&mut data)
      .expect("Could not read from stdin");
    data[0]
  }

  fn write_output(&mut self, output: CellValue) {
    stdout()
      .write_all(&[output])
      .expect("Could not write to stdout");
  }
}
