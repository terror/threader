use crate::common::*;

pub trait ParserExt<'a> {
  fn between(&self, tag: Tag) -> Vec<String>;
}

impl<'a> ParserExt<'a> for Parser<'a> {
  fn between(&self, tag: Tag) -> Vec<String> {
    let mut inside = false;
    let mut ret = Vec::new();

    for event in self {
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
