use crate::common::*;

/// The `Parser` struct holds the content to parse `content` as a string and
/// uses the `MarkdownParser` struct provided by the pulldown_cmark crate
/// internally to parse that content.

#[derive(Debug, Clone)]
pub struct Parser<'a> {
  content: &'a str,
}

impl<'a> Parser<'a> {
  pub fn new(content: &'a str) -> Self {
    Self {
      content,
    }
  }

  /// Extracts text in between a given `Tag` using a newly created
  /// `MarkdownParser` instance.
  pub fn extract_between(&self, tag: Tag) -> Vec<String> {
    let parser = MarkdownParser::new(self.content);

    let mut inside = false;
    let mut ret = Vec::new();

    for event in parser {
      match event {
        Event::Start(start_tag) => {
          if start_tag == tag {
            inside = true;
          }
        }
        Event::End(end_tag) => {
          if end_tag == tag {
            inside = false;
          }
        }
        Event::Text(text) => {
          if inside {
            ret.push(text.to_string());
          }
        }
        _ => {}
      }
    }

    ret
  }
}
