use crate::syntax::Instruction;
use core::fmt;
use lazy_regex::{Captures, Lazy, Regex};

mod break_token;
mod decrement;
mod goto;
mod if_token;
mod increment;
mod loop_token;
mod move_token;
mod print;
mod read;
mod save;
mod swap;
mod write;

pub use break_token::*;
pub use decrement::*;
pub use goto::*;
pub use if_token::*;
pub use increment::*;
pub use loop_token::*;
pub use move_token::*;
pub use print::*;
pub use read::*;
pub use save::*;
pub use swap::*;
pub use write::*;

pub trait Token: fmt::Debug {
  /// The name of the token.
  fn name(&self) -> &'static str;

  /// The regex used to match the token.
  fn regex(&self) -> &'static Lazy<Regex>;

  /// Parses a line of code into a `Instruction`.
  ///
  /// Returns the next line to parse and maybe a instruction.
  fn read_instruction(
    &self,
    file: &[&str],
    captures: Captures,
    line_index: usize,
    indentation: usize,
  ) -> (usize, Option<Instruction>);
}

impl<'a> PartialEq<&'a dyn Token> for &'a dyn Token {
  fn eq(&self, other: &&'a dyn Token) -> bool {
    self.name() == other.name()
  }
}

pub fn all_tokens() -> [&'static dyn Token; 12] {
  [
    &decrement::DecrementToken,
    &goto::GotoToken,
    &if_token::IfToken,
    &increment::IncrementToken,
    &loop_token::LoopToken,
    &move_token::MoveToken,
    &print::PrintToken,
    &read::ReadToken,
    &save::SaveToken,
    &swap::SwapToken,
    &write::WriteToken,
    &break_token::BreakToken,
  ]
}
