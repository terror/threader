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

#[cfg(test)]
pub fn strip(s: String) -> String {
  dedent(s.strip_prefix('\n').unwrap())
}

pub fn create_file(path: &Path, content: &str) -> Result<()> {
  let mut file = FSFile::create(path)?;
  file.write_all(content.as_bytes())?;
  Ok(())
}
