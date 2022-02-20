use brainease_lexer::syntax::CellValue;

mod default;
pub use default::*;

pub trait IoHandler: Clone {
  type Err;

  fn read_input(&mut self) -> Result<CellValue, Self::Err>;
  fn write_output(&mut self, output: &[CellValue]) -> Result<(), Self::Err>;

  /// Flushes the output buffer.
  ///
  /// Normally just emits a newline character.
  fn flush(&mut self) -> Result<(), Self::Err>;
}
