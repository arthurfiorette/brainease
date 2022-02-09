use std::io::{stdin, stdout, Read, Write};

use brainease_lexer::syntax::CellValue;

pub trait IoHandler {
  fn read_input(&mut self) -> CellValue;
  fn write_output(&mut self, output: CellValue);
}

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

#[derive(Debug, Clone)]
pub struct ProxyIoHandler {
  read_byte: fn() -> CellValue,
  write_byte: fn(CellValue) -> (),
}

impl IoHandler for ProxyIoHandler {
  fn read_input(&mut self) -> CellValue {
    (self.read_byte)()
  }

  fn write_output(&mut self, output: CellValue) {
    (self.write_byte)(output)
  }
}
