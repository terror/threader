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

    let parser = Parser::new(&content);

    let title = &parser.between(Tag::Heading(2)).first().cloned();

    let tweets = &parser
      .between(Tag::BlockQuote)
      .iter()
      .enumerate()
      .map(|(i, item)| Tweet {
        id:   i as i64,
        text: item.to_string(),
      })
      .collect::<Vec<Tweet>>();

    Ok(Thread::new(title.to_owned(), tweets.to_owned()))
  }
}
