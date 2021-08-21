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

  pub fn parse(&self) -> Result<Vec<Tweet>> {
    let content = fs::read_to_string(&self.path)?;

    let parser = Parser::new(&content);

    let title = parser.between(Tag::Heading(2)).first().cloned();

    let paragraphs = parser.between(Tag::Paragraph);

    let mut tweets = Vec::new();

    for (i, content) in paragraphs.iter().enumerate() {
      let index = i as i64;
      match index {
        0 => tweets.push(Tweet::new(
          Prefix::new(index + 1, paragraphs.len() as i64, &title),
          content.to_string(),
        )?),
        _ => tweets.push(Tweet::new(
          Prefix::new(index + 1, paragraphs.len() as i64, &None),
          content.to_string(),
        )?),
      }
    }

    Ok(tweets)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    in_temp_dir!({
      let path = env::current_dir().unwrap().join("thread.md");

      create_file(
        &path,
        &strip(
        r#"
          ## Thread

          Tweet A

          Tweet B

          Tweet C
      "#
          .into(),
        ),
      )
      .unwrap();

      let file = File::new(&path);

      let tweets = file.parse();

      assert!(tweets.is_ok());
      assert_eq!(tweets.unwrap().len(), 3);
    });
  }

  #[test]
  fn exceed_character_limit() {
    in_temp_dir!({
      let path = env::current_dir().unwrap().join("thread.md");

      create_file(
        &path,
        &strip(
        r#"
          ## Thread

          Cool stuff bro

          This tweet exceeds Twitter's character limit.  Writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space.
      "#
          .into(),
        ),
      )
      .unwrap();

      let file = File::new(&path);

      assert!(file.parse().is_err());
    });
  }
}
