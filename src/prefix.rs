use crate::common::*;

#[derive(Debug, Clone)]
pub struct Prefix<'a> {
  id:     i64,
  length: i64,
  title:  &'a Option<String>,
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

impl fmt::Display for Prefix<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if let Some(title) = self.title {
      return write!(f, "{}\n\n{}/{} ", title, self.id, self.length);
    }
    write!(f, "{}/{}", self.id, self.length)
  }
}
