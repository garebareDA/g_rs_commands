use super::rm_commands::RmCommands;
use std::path::Path;

impl RmCommands {
  pub (crate) fn remove_file<P: AsRef<Path>>(&self, path:P) -> Result<(), String> {
    let path = path.as_ref();
    if path.exists() {
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
}