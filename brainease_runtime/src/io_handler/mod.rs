mod default;
mod proxy;

pub use default::*;
pub use proxy::*;

use brainease_lexer::syntax::CellValue;

pub trait IoHandler {
  fn read_input(&mut self) -> CellValue;
  fn write_output(&mut self, output: CellValue);
}
