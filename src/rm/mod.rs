pub mod rm_commands;
pub mod rm_file;


#[cfg(test)]
pub mod tests {
  use crate::parser::commands::Commands;
  use crate::parser::command_line::CommandLine;
  use crate::rm::rm_commands::RmCommands;
  use std::fs;
  use std::fs::File;

  #[test]
  fn rm_file() {
    File::create("test.txt").unwrap();
    let mut command_line = CommandLine::new();
    command_line.set_name("rm");
    command_line.set_args(vec!["test.txt".into()]);
    command_line.set_options(vec![]);
    let rm_commands = RmCommands::new(command_line);
    match rm_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn rm_dir() {
    fs::create_dir("test_dir").unwrap();
    let mut command_line = CommandLine::new();
    command_line.set_name("rm");
    command_line.set_args(vec!["test_dir".into()]);
    command_line.set_options(vec![]);
    let rm_commands = RmCommands::new(command_line);
    match rm_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }
}