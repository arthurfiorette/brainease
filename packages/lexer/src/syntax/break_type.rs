use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BreakType {
    Exit,
    Break,
    BreakAll,
    Continue,
}

impl FromStr for BreakType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exit" => Ok(BreakType::Exit),
            "break" => Ok(BreakType::Break),
            "break all" => Ok(BreakType::BreakAll),
            "continue" => Ok(BreakType::Continue),
            _ => Err(()),
        }
    }
}

impl ToString for BreakType {
    fn to_string(&self) -> String {
        match self {
            BreakType::Exit => "exit".to_string(),
            BreakType::Break => "break".to_string(),
            BreakType::BreakAll => "break all".to_string(),
            BreakType::Continue => "continue".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::syntax::BreakType;

    #[test]
    fn string_parse() {
        let break_types = ["exit", "break", "continue", "break all"];

        for break_type in break_types.iter() {
            assert_eq!(
                break_type,
                &break_type.parse::<BreakType>().unwrap().to_string()
            );

            assert_eq!(
                break_type,
                &BreakType::from_str(break_type).unwrap().to_string()
            );
        }
    }
}
