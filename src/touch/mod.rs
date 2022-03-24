pub mod touch_commands;
pub mod touch_file;

#[cfg(test)]
pub mod tests {
  use filetime::FileTime;

use crate::parser::commands::Commands;
  use crate::parser::command_line::CommandLine;
  use crate::touch::touch_commands::TouchCommands;
  use std::path::Path;
  use std::thread;
  use std::fs;

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

  #[test]
  fn touch_file() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test.txt").exists());
    thread::sleep(std::time::Duration::from_secs(1));

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    let a = FileTime::from_last_access_time(&meta);

    assert_eq!(m.seconds(), FileTime::now().seconds());
    assert_eq!(a.seconds(), FileTime::now().seconds());
    fs::remove_file("test.txt").unwrap();
  }

  #[test]
  fn touch_file_modification() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test.txt").exists());
    thread::sleep(std::time::Duration::from_secs(1));

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test.txt".into()]);
    command_line.set_options(vec!["-m".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    let a = FileTime::from_last_access_time(&meta);

    assert_eq!(m.unix_seconds(), FileTime::now().unix_seconds());
    assert_ne!(a.unix_seconds(), FileTime::now().unix_seconds());
    fs::remove_file("test.txt").unwrap();
  }
}