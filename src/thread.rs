use crate::common::*;

#[derive(Debug, Default, Clone)]
pub struct Thread {
  title:  Option<String>,
  tweets: Vec<Tweet>,
  length: i64,
}

impl Thread {
  pub fn new(title: Option<String>, tweets: Vec<Tweet>) -> Self {
    let length = tweets.len() as i64;
    Self {
      title,
      tweets,
      length,
    }
  }

  pub fn title(&self) -> Option<String> {
    self.title.clone()
  }

  pub fn tweets(&self) -> Vec<Tweet> {
    self.tweets.clone()
  }

  pub fn length(&self) -> i64 {
    self.length
  }
}
