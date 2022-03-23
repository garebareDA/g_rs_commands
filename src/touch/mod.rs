pub mod touch_commands;
pub mod touch_file;

#[cfg(test)]
pub mod tests {
  use crate::parser::commands::Commands;
  use crate::parser::command_line::CommandLine;
  use crate::touch::touch_commands::TouchCommands;
  use std::fs;
use std::path::Path;

  #[test]
  fn touch_crete_file() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test.txt".into()]);
    command_line.set_options(vec![]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test.txt").exists());
    fs::remove_file("test.txt").unwrap();
  }
}