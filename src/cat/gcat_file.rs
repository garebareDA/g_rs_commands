use std::path::Path;
use std::io::Read;

use super::gcat_commands::CatCommands;

impl CatCommands {
  pub fn print_files<P:AsRef<Path>>(&self, paths:&Vec<P>) -> Result<(), String> {
    for path in paths.iter() {
      println!("{}", self.read_file(path.as_ref())?);
    }
    return Ok(());
  }

  fn read_file<P:AsRef<Path>>(&self, path: P) -> Result<String, String> {
    let path = path.as_ref();
    let mut file = std::fs::File::open(path).ok().ok_or(format!("gcat:{} is no such file", path.display()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok().ok_or("gcat: error reading file")?;
    return Ok(contents);
  }
}