use crate::common::*;

#[macro_export]
macro_rules! in_temp_dir {
  ($body: block) => {
    let tempdir = TempDir::new().unwrap();
    assert!(tempdir.path().exists());
    env::set_current_dir(&tempdir.path()).unwrap();
    $body
  };
}

/// Creates a new file given a reference to a `Path` instance `path` and the
/// files content `content` to be written to the file.
pub fn create_file_with_content<'a>(path: &'a Path, content: &str) -> &'a Path {
  let mut file = std::fs::File::create(path).unwrap();
  file.write_all(content.as_bytes()).unwrap();
  path
}
