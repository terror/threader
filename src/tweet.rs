use crate::common::*;

const LIMIT: usize = 280;

#[derive(Debug, Clone)]
pub struct Tweet {
  id:   i64,
  text: String,
}

impl Tweet {
  pub fn new(id: i64, text: String) -> Result<Self> {
    let size = text.chars().count();

    if size > LIMIT {
      return Err(Error::CharacterLimit {
        over_by: size - LIMIT,
        content: text,
      });
    }

    Ok(Tweet {
      id,
      text,
    })
  }

  pub fn add_title(&self, title: Option<String>) -> String {
    if let Some(title) = title {
      return format!("{}\n\n{}/ {}", title, self.id, self.text);
    }
    format!("{}/ {}", self.id, self.text)
  }
}

impl fmt::Display for Tweet {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}/ {}", self.id, self.text)
  }
}
