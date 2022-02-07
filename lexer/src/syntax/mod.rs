mod if_logic;
mod instruction;
mod parser;
mod regex;
mod token;

pub use if_logic::*;
pub use instruction::*;
pub use parser::*;
pub use regex::*;
pub use token::*;

pub type CellPosition = usize;
pub type CellValue = u8;
