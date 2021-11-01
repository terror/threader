use crate::common::*;

/// A `Prefix` is what is displayed before a tweets content. For instance, 1/4
/// This is a tweet; here `1/4` is the prefix. If the prefix contains a `title`,
/// which is not None, it is only displayed in the first tweet in the thread
/// delimited by an extra newline.

#[derive(Debug, Clone)]
pub struct Prefix<'a> {
  id:     i64,
  length: i64,
  title:  &'a Option<String>,
}

impl Display for Prefix<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    if let Some(title) = self.title {
      return write!(f, "{}\n\n{}/{} ", title, self.id, self.length);
    }
    write!(f, "{}/{}", self.id, self.length)
  }
}

impl<'a> Prefix<'a> {
  pub fn new(id: i64, length: i64, title: &'a Option<String>) -> Self {
    Self {
      id,
      length,
      title,
    }
  }
}
