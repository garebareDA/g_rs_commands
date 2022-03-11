use super::ls_commands::LsCommands;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub(crate) struct File {
  name: String,
  is_dir: bool,
}

impl File {
  pub fn new(name: String, is_dir: bool) -> File {
    File { name, is_dir }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn is_dir(&self) -> bool {
    self.is_dir
  }
}

impl LsCommands {
  pub(crate) fn read_dir<P: AsRef<Path>>(&self, path: P) -> Vec<File> {
    let files = fs::read_dir(path)
      .unwrap()
      .map(|entry| {
        let entry = entry.unwrap();
        return File::new(
          entry.file_name().into_string().unwrap(),
          entry.path().is_dir(),
        );
      })
      .collect();

    println!("{:?}", files);
    return files;
  }
}
