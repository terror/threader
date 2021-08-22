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

  #[rstest]
  #[case("", 0)]
  #[case("hello, world!", 1)]
  #[case(
    r#"
    ## Title

    Tweet A

    Tweet B

    Tweet C
  "#,
    3
  )]
  fn parse(#[case] content: &str, #[case] length: usize) {
    in_temp_dir!({
      let path = env::current_dir().unwrap().join("thread.md");

      create_file(&path, &strip(content.to_owned())).unwrap();

      println!("{}", content);

      let tweets = File::new(&path).parse();

      println!("{:?}", tweets);

      assert!(tweets.is_ok());
      assert_eq!(tweets.unwrap().len(), length);
    });
  }

  #[rstest]
  #[case(r#"
    ## Thread

    Cool stuff bro

    This tweet exceeds Twitter's character limit.  Writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space.
  "#)]
  fn exceed_character_limit(#[case] content: &str) {
    in_temp_dir!({
      let path = env::current_dir().unwrap().join("thread.md");

      create_file(&path, &strip(content.to_string())).unwrap();

      let tweets = File::new(&path).parse();

      assert!(tweets.is_err());
    });
  }
}
