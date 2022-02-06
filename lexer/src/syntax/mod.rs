mod if_logic;
mod instruction;
mod token;

pub use if_logic::*;
pub use instruction::*;
pub use token::*;

pub type CellPosition = usize;
pub type CellValue = u8;
