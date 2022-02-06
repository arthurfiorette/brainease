use log;

pub enum Messages {
  HeaderNeedsValue {
    name: String,
    line: String,
  },
  HeaderInvalidValue {
    name: String,
    line: String,
    required: String,
  },
  HeaderUnrecognized {
    name: String,
    line: String,
  },
}

impl Messages {
  pub fn log_error(&self) {
    log::error!("{}", self.to_string());
  }
  pub fn log_warning(&self) {
    log::warn!("{}", self.to_string());
  }
}

impl ToString for Messages {
  fn to_string(&self) -> String {
    match self {
      Messages::HeaderNeedsValue { name, line } => {
        format!(
          "Header '{}' needs a value. Ignoring it. (Line {})",
          name, line
        )
      }
      Messages::HeaderInvalidValue {
        name,
        line,
        required,
      } => {
        format!(
          "Header '{}' needs a value of type '{}'. Ignoring it. (Line {})",
          name, required, line
        )
      }
      Messages::HeaderUnrecognized { name, line } => {
        format!("Unrecognized header '{}'. (Line {})", name, line)
      }
    }
  }
}
