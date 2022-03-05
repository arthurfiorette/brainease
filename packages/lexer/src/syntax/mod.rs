mod find_match;
mod goto;
mod if_logic;
mod instruction;
mod pointer;

pub use find_match::*;
pub use goto::*;
pub use if_logic::*;
pub use instruction::*;
pub use pointer::*;

pub type CellPosition = usize;
pub type CellValue = u8;
