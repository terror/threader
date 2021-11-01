use crate::common::*;

/// The `File` struct is successfully constructed given a path to a markdown
/// file `path` and provides a single method `parse` able of parsing the
/// contents of the file into a list of `Tweet` instances.

#[derive(Debug)]
pub struct File<'a> {
  path: &'a Path,
}

impl<'a> File<'a> {
  /// Creates a new instance of `File` ensuring that the given path `path` is a
  /// file that contains the appropriate extension, '.md'.
  pub fn new(path: &'a Path) -> Result<Self> {
    if let Some(extension) = path.extension() {
      if extension == MARKDOWN {
        return Ok(Self {
          path,
        });
      }
    }
    Err(Error::ConstructFile {
      path: path.to_owned(),
    })
  }

  /// Parses the contents of the given markdown file path and returns the
  /// content in the form of a `Vec<Tweet>`, tweetable by a `Client` instance.
  pub fn parse(&self) -> Result<Vec<Tweet>> {
    let content = fs::read_to_string(&self.path)?;

    let parser = Parser::new(&content);

    let title = parser.extract_between(Tag::Heading(2)).first().cloned();
    let paragraphs = parser.extract_between(Tag::Paragraph);

    let mut tweets = Vec::new();

    for (i, content) in paragraphs.iter().enumerate() {
      let index = i as i64;
      tweets.push(Tweet::new(
        Prefix::new(
          index + 1,
          paragraphs.len() as i64,
          if index == 0 { &title } else { &None },
        ),
        content.to_string(),
      )?)
    }

    Ok(tweets)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  const FILE_PATH: &str = "thread.md";

  struct Fixture {
    content: &'static str,
    length:  usize,
    fail:    bool,
  }

  fn fixtures() -> Vec<Fixture> {
    vec![
      Fixture {
        content: "",
        length:  0,
        fail:    false,
      },
      Fixture {
        content: "hello, world!",
        length:  1,
        fail:    false,
      },
      Fixture {
        content: indoc! {"
      ## Title

      Tweet A

      Tweet B

      Tweet C
    "},
        length:  3,
        fail:    false,
      },
      Fixture {
        content: indoc! {"
      ## Thread

      Threader is cool!

      This tweet exceeds Twitter's character limit.  Writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space, writing to fill up space.
    "},
        length:  3,
        fail:    true,
      },
    ]
  }

  #[test]
  fn parse() {
    for fixture in fixtures() {
      in_temp_dir!({
        let tweets = File::new(create_file_with_content(
          &env::current_dir().unwrap().join(FILE_PATH),
          fixture.content,
        ))
        .unwrap()
        .parse();

        if fixture.fail {
          assert!(tweets.is_err());
        } else {
          assert!(tweets.is_ok());
          assert_eq!(tweets.unwrap().len(), fixture.length);
        }
      });
    }
  }
}
