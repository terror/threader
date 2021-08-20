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

  pub fn to_string(&self, title: Option<String>, thread_length: i64) -> String {
    if let Some(title) = title {
      return format!(
        "{}\n\n{}/{} {}",
        title, self.id, thread_length, self.text
      );
    }
    format!("{}/{} {}", self.id, thread_length, self.text)
  }
}
