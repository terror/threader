use crate::common::*;

/// A `Tweet` instance simply stores the content of the tweet, and includes
/// length validation upon calls to the constructor `Tweet::new()`, taking into
/// account Twitter's character limit of 280.

#[derive(Debug, Clone)]
pub struct Tweet {
  content: String,
}

impl Tweet {
  pub fn new(prefix: Prefix, content: String) -> Result<Self> {
    let content = format!("{} {}", prefix, content);

    let size = content.chars().count();

    if size > CHARACTER_LIMIT {
      return Err(Error::CharacterLimit {
        over_by: size - CHARACTER_LIMIT,
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
