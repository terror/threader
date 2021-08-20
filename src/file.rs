use crate::common::*;

#[derive(Debug)]
pub struct File<'a> {
  path: &'a PathBuf,
}

impl<'a> File<'a> {
  pub fn new(path: &'a PathBuf) -> Self {
    Self {
      path,
    }
  }

  pub fn parse(&self) -> Result<Thread> {
    let content = fs::read_to_string(&self.path)?;

    let title = between(&content, Tag::Heading(2)).first().cloned();

    let mut tweets = Vec::new();
    for (i, content) in between(&content, Tag::BlockQuote).iter().enumerate() {
      let tweet = Tweet::new((i + 1) as i64, content.to_string())?;
      tweets.push(tweet);
    }

    Ok(Thread::new(title, tweets))
  }
}
