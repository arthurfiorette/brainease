use super::{CellOrPointer, CellValue, GotoBy, GotoDirection, IfLogic, TokenKind};

/// A Instruction that contain brainease logic.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
  /// Increment the value at the given position.
  ///
  /// ```r
  /// # Increments cell 10 by 1
  /// inc 1 in 10
  /// ```
  Increment {
    cell: CellOrPointer,
    value: CellValue,
  },

  /// Decrement the value at the given cell.
  ///
  /// ```r
  /// # Decrements cell 10 by 1
  /// dec 7 in 43
  /// ```
  Decrement {
    cell: CellOrPointer,
    value: CellValue,
  },

  /// Moves the value of the cell to next cell.
  /// The current cell will end up resetting to 0.
  ///
  /// ```r
  /// # Moves cell 1 to @2
  /// move # to 2
  /// ```
  Move {
    current: CellOrPointer,
    next: CellOrPointer,
  },

  /// Swap the value of two cells
  ///
  /// ```r
  /// # Swap the cell 4 with cell 5
  /// swap 4 with 5
  /// ```
  Swap {
    from: CellOrPointer,
    with: CellOrPointer,
  },

  /// Save the char ASCII value at the given cell
  ///
  /// ```r
  /// # Saves the ASCII code of J (which is 74) at cell 10
  /// save 'J' in 10
  /// ```
  Save {
    cell: CellOrPointer,
    value: CellValue,
  },

  /// Reads the ASCII code of the given char from stdin.
  ///
  /// ```r
  /// # Saves the ASCII code of the given input at cell 10
  /// read 10
  /// ```
  Read(CellOrPointer),

  /// Sends the current cell value to stdout.
  ///
  /// ```r
  /// # Prints the byte value of cell 10 to stdout
  /// write 10
  /// ```
  Write(CellOrPointer),

  /// Sends the current ASCII code of the given char to stdout.
  ///
  /// ```r
  /// # Prints the ASCII code stored in cell 10
  /// print 10
  /// ```
  Print(CellOrPointer),

  /// Loops the inner instructions until the given cell value is 0
  ///
  /// ```r
  /// # Prints 10987654321 to stdout
  ///
  /// # Increments 10 in cell 1
  /// inc 10 in 1
  ///
  /// # Repeat the loop code until cell 1 is 0
  /// loop 1
  ///   # Sends current cell 1 value to stdout
  ///   write 1
  /// ```
  Loop {
    cell: CellOrPointer,
    inner: Vec<Instruction>,
  },

  /// Executes the inner instructions if the specified cell matches
  /// the logic with the given value
  ///
  /// ```r
  /// # Adds 10 to cell 1
  /// inc 10 in 1
  ///
  /// # Prints 10 to STDOUT if cell 1 is lower than 5
  /// if 1 < 5
  ///   write 1
  ///
  /// ```
  If {
    cell: CellOrPointer,
    cell_or_value: CellOrPointer,
    /// If the cellOrValue points to a cell instead of a value
    is_cell: bool,
    logic: IfLogic,
    inner: Vec<Instruction>,
  },

  /// Moves the current pointer to another cell
  ///
  /// ```r
  /// # Moves the current pointer by two cells to left
  /// goto left by 2
  ///
  /// # Moves the current pointer by (value of cell 2) cells to right
  /// goto right by *2
  /// ```
  Goto {
    dir: GotoDirection,
    by: Option<GotoBy>,
  },
}

impl Instruction {
  pub fn kind(&self) -> TokenKind {
    match self {
      Instruction::Increment { .. } => TokenKind::Increment,
      Instruction::Decrement { .. } => TokenKind::Decrement,
      Instruction::Move { .. } => TokenKind::Move,
      Instruction::Swap { .. } => TokenKind::Swap,
      Instruction::Save { .. } => TokenKind::Save,
      Instruction::Read(_) => TokenKind::Read,
      Instruction::Write(_) => TokenKind::Write,
      Instruction::Print(_) => TokenKind::Print,
      Instruction::Loop { .. } => TokenKind::Loop,
      Instruction::If { .. } => TokenKind::If,
      Instruction::Goto { .. } => TokenKind::Goto,
    }
  }
}
