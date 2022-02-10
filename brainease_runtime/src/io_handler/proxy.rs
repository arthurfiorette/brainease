use brainease_lexer::syntax::CellValue;

use super::IoHandler;

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
