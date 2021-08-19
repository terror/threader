use crate::common::*;

pub fn between(content: &str, tag: Tag) -> Vec<String> {
  let parser = Parser::new(content);

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
