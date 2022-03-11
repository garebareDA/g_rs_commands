use super::ls_commands::LsCommands;
use std::fs;
use std::path::Path;

struct File {
    name: String,
    is_dir: bool,
}

impl File {
    pub fn new (name: String, is_dir: bool) -> File {
        File {
            name,
            is_dir,
        }
    }
}

impl LsCommands {
  pub(crate) fn read_dir<P: AsRef<Path>>(&self, path:P) -> Vec<String> {
    let files = fs::read_dir(path).unwrap().map(|entry|{
      let entry = entry.unwrap();
      return entry.file_name().into_string().unwrap();
    }).collect();

    println!("{:?}", files);
    return files;
  }
}