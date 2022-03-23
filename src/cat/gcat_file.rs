use std::io::BufRead;
use std::path::Path;

use super::gcat_commands::CatCommands;

impl CatCommands {
    pub fn print_files<P: AsRef<Path>>(&self, paths: &Vec<P>) -> Result<(), String> {
        let mut line_number = 0;
        for path in paths.iter() {
            let end = if self.get_show_end() { "$" } else { "" };
            for line in self.read_file(path.as_ref())?.iter() {
                if line == "" && self.get_hide_blank() {
                    continue;
                }
                line_number += 1;
                let number = if self.get_show_number() {
                    format!("{} ", line_number)
                } else {
                    "".to_string()
                };
                println!("{}{}{}", number, line, end);
            }
        }
        return Ok(());
    }

    fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<String>, String> {
        let path = path.as_ref();
        let mut file = std::fs::File::open(path)
            .ok()
            .ok_or(format!("gcat:{} is no such file", path.display()))?;
        let buffer = std::io::BufReader::new(&mut file);
        let mut contents: Vec<String> = Vec::new();
        for line in buffer.lines() {
            contents.push(
                line.ok()
                    .ok_or(format!("gcat: {}: read error", path.display()))?,
            );
        }
        return Ok(contents);
    }
}
