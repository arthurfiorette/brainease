use std::io::{self, stdin, stdout, Read, Write};

use brainease_lexer::syntax::CellValue;

use super::IoHandler;

#[derive(Debug, Clone)]
pub struct DefaultIoHandler {}

impl IoHandler for DefaultIoHandler {
  type Err = io::Error;

  fn read_input(&mut self) -> Result<CellValue, Self::Err> {
    println!("Enter input: ");

    let mut data = [0; 1];
    stdin().read_exact(&mut data)?;

    Ok(data[0])
  }

  fn write_output(&mut self, output: &[CellValue]) -> Result<(), Self::Err> {
    stdout().write_all(output)
  }

  fn flush(&mut self) -> Result<(), Self::Err> {
    stdout().flush()
  }
}
