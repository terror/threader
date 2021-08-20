use crate::common::*;

#[derive(Debug)]
pub struct File<'a> {
  path: &'a Path,
}

impl<'a> File<'a> {
  pub fn new(path: &'a Path) -> Self {
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

#[cfg(test)]
mod tests {
  use super::*;

  fn strip(s: String) -> String {
    dedent(s.strip_prefix('\n').unwrap())
  }

  #[test]
  fn exceed_character_limit() {
    in_temp_dir!({
      let directory = env::current_dir().unwrap().join("thread.md");

      create_file(
        &directory,
        &strip(r#"
          ## Thread

          > Cool stuff bro

          > This tweet exceeds Twitter's character limit. Writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up spaceeeeeeeee.
      "#.into())).unwrap();

      let file = File::new(&directory);

      assert!(file.parse().is_err());
    });
  }
}
