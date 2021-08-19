use crate::common::*;

#[derive(Debug, Clone)]
pub struct Tweet {
  id:   i64,
  text: String,
}

impl Tweet {
  pub fn new(id: i64, text: String) -> Self {
    Tweet {
      id,
      text,
    }
  }

  pub fn add_title(&self, title: Option<String>) -> Self {
    if let Some(title) = title {
      return Tweet {
        id:   self.id,
        text: format!("{}\n\n{}", title, self.text),
      };
    }
    self.clone()
  }
}

impl fmt::Display for Tweet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}/ {}", self.id, self.text)
  }
}
