use crate::common::*;

const LIMIT: usize = 280;

#[derive(Debug, Clone)]
pub struct Tweet {
  content: String,
}

impl Tweet {
  pub fn new(prefix: Prefix, content: String) -> Result<Self> {
    let content = format!("{} {}", prefix, content);

    let size = content.chars().count();

    if size > LIMIT {
      return Err(Error::CharacterLimit {
        over_by: size - LIMIT,
        content,
      });
    }

    Ok(Tweet {
      content,
    })
  }

  pub fn content(&self) -> String {
    self.content.clone()
  }
}
