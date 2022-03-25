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

        return Err(format!(
            "grm: cannot remove {} : No such a file or directory",
            path.display()
        ));
    }

    pub fn remove_reverso<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();
        if path.exists() {
            if path.is_dir() {
                self.remove_visit_dir(path)?;
                self.remove_file(path)?;
                return Ok(());
            } else {
                return Err(format!(
                    "grm: cannot remove {} : No directory",
                    path.display()
                ));
            }
        }
        return Err(format!(
            "grm: cannot remove {} : No such a file or directory",
            path.display()
        ));
    }

    fn remove_visit_dir<P: AsRef<Path> + Copy>(&self, path: P) -> Result<(), String> {
        for entry in std::fs::read_dir(path).ok().ok_or(format!(
            "grm: cannot remove {} : No such a file or directory",
            path.as_ref().display()
        ))? {
            println!("a{}", path.as_ref().display());
            let entry = entry.ok().ok_or(format!(
                "grm: cannot remove {} : No such a file or directory",
                path.as_ref().display()
            ))?;
            let path = entry.path();
            println!("b{}", path.display());
            if path.is_dir() {
                self.remove_visit_dir(&path)?;
            }
            self.remove_file(&path)?;
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
