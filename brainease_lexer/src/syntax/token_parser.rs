use lazy_regex::Captures;

use super::{GotoDirection, IfLogic, Instruction, GotoBy};
use crate::{logger, parser::parse_partial_file, util};

/// A closure that parses a line of code into a `Instruction`.
///
/// Returns the next line to parse and the optional instruction.
pub type TokenParser = fn(
  file: &[&str],
  captures: Captures,
  line_index: usize,
  indentation: usize,
) -> (usize, Option<Instruction>);

// A token parser for the Increment instruction regex result.
pub static INCREMENT: TokenParser = |_, captures, line_index, _| {
  let val = captures[1].parse().unwrap();
  let cell = captures[2].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[3]);

  if val > u8::MAX as usize {
    logger::value_too_big(&line_index, &val, u8::MAX as usize);
    return (line_index + 1, None);
  }

  (
    line_index + 1,
    Some(Instruction::Increment {
      cell,
      value: val as u8,
    }),
  )
};

// A token parser for the Decrement instruction regex result.
pub static DECREMENT: TokenParser = |_, captures, line_index, _| {
  let val = captures[1].parse().unwrap();
  let cell = captures[2].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[3]);

  if val > u8::MAX as usize {
    logger::value_too_big(&line_index, &val, u8::MAX as usize);
    return (line_index + 1, None);
  }

  (
    line_index + 1,
    Some(Instruction::Decrement {
      cell,
      value: val as u8,
    }),
  )
};

// A token parser for the Move instruction regex result.
pub static MOVE: TokenParser = |_, captures, line_index, _| {
  let current = captures[1].parse().unwrap();
  let next = captures[2].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[3]);

  (line_index + 1, Some(Instruction::Move { current, next }))
};

// A token parser for the Swap instruction regex result.
pub static SWAP: TokenParser = |_, captures, line_index, _| {
  let from = captures[1].parse().unwrap();
  let with = captures[2].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[3]);

  (line_index + 1, Some(Instruction::Swap { from, with }))
};

// A token parser for the Save instruction regex result.
pub static SAVE: TokenParser = |_, captures, line_index, _| {
  let char = captures[1].chars().next().unwrap();
  let cell = captures[2].parse().unwrap();

  if char as usize > u8::MAX as usize {
    logger::invalid_char(&line_index, &char);
    panic!()
  }

  util::log_extra_chars(&line_index, &captures[3]);

  (
    line_index + 1,
    Some(Instruction::Save {
      cell,
      value: char as u8,
    }),
  )
};

// A token parser for the Read instruction regex result.
pub static READ: TokenParser = |_, captures, line_index, _| {
  let cell = captures[1].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[2]);

  (line_index + 1, Some(Instruction::Read(cell)))
};

// A token parser for the Write instruction regex result.
pub static WRITE: TokenParser = |_, captures, line_index, _| {
  let cell = captures[1].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[2]);

  (line_index + 1, Some(Instruction::Write(cell)))
};

// A token parser for the Print instruction regex result.
pub static PRINT: TokenParser = |_, captures, line_index, _| {
  let cell = captures[1].parse().unwrap();

  util::log_extra_chars(&line_index, &captures[2]);

  (line_index + 1, Some(Instruction::Print(cell)))
};

// A token parser for the Loop instruction regex result.
pub static LOOP: TokenParser = |file, captures, line_index, indentation| {
  let cell = captures[1].parse().unwrap();

  // Read and parses the inner indentation block
  let (next_line, inner) =
    parse_partial_file(file, line_index + 1, indentation + 2, line_index);

  (next_line, Some(Instruction::Loop { cell, inner }))
};

// A token parser for the If instruction regex result.
pub static IF: TokenParser = |file, captures, line_index, indentation| {
  IfLogic::parse(file, captures, line_index, indentation)
};

// A token parser for the Goto instruction regex result.
pub static GOTO: TokenParser = |_, captures, line_index, _| {
  let dir: GotoDirection = captures[1].parse().unwrap();

  let by: Option<GotoBy> = captures
    .get(2)
    .and_then(|by_value| by_value.as_str().parse().ok());

  (line_index + 1, Some(Instruction::Goto { dir, by }))
};
