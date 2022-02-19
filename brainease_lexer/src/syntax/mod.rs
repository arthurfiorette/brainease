mod goto;
mod if_logic;
mod instruction;
mod pointer;
mod regex;
mod token;
mod token_parser;

pub use goto::*;
pub use if_logic::*;
pub use instruction::*;
pub use pointer::*;
pub use regex::*;
pub use token::*;
pub use token_parser::*;

pub type CellPosition = usize;
pub type CellValue = u8;
