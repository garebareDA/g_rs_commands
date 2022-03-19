use super::rm_commands::RmCommands;
use std::path::Path;

impl RmCommands {
  pub(crate) fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
      if !self.remove_interactive() {
        return Ok(());
      }

      if path.is_file() {
        return std::fs::remove_file(path).ok().ok_or(format!(
          "grm: cannot remove '{}' : Permission denied",
          path.display()
        ));
      } else {
        return std::fs::remove_dir(path).ok().ok_or(format!(
          "grm: cannot remove '{}' : Permission denied",
          path.display()
        ));
      }
    }

    return Err("grm: cannot remove '{}' : No such a file or directory".to_string());
  }

  pub fn remove_reverso<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
      if path.is_dir() {
        self.remove_visit_dir(path)?;
      } else {
        return Err("grm -r: cannot remove '{}' : No directory".to_string());
      }
    }
    return Err("grm: cannot remove '{}' : No such a file or directory".to_string());
  }

  fn remove_visit_dir<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
    for entry in std::fs::read_dir(path).unwrap() {
      let entry = entry
        .ok()
        .ok_or("grm: cannot remove '{}' : No such a file or directory".to_string())?;
      let path = entry.path();
      if path.is_dir() {
        self.remove_visit_dir(&path)?;
        self.remove_file(&path)?;
      } else {
        self.remove_file(&path)?;
      }
    }
    return Ok(());
  }

  fn remove_interactive(&self) -> bool {
    if self.get_interactive() {
      loop {
        println!("{}", "Are you sure you want to delete this file? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "n" {
          return false;
        } else if input.trim() == "y" {
          break;
        }
      }
    }
    return true;
  }
}
