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
    command_line.set_args(vec!["test1.txt".into()]);
    command_line.set_options(vec![]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test1.txt").exists());
    fs::remove_file("test1.txt").unwrap();
  }

  #[test]
  fn touch_file() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test2.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test2.txt").exists());
    let meta = fs::metadata("test2.txt").unwrap();
    let mb = FileTime::from_last_modification_time(&meta);
    let ab = FileTime::from_last_access_time(&meta);
    thread::sleep(std::time::Duration::from_secs(1));

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test2.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test2.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    let a = FileTime::from_last_access_time(&meta);
    fs::remove_file("test2.txt").unwrap();

    assert_ne!(m.seconds(), mb.seconds());
    assert_ne!(a.seconds(), ab.seconds());
  }

  #[test]
  fn touch_file_modification() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test3.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test3.txt").exists());
    let meta = fs::metadata("test3.txt").unwrap();
    let mb = FileTime::from_last_modification_time(&meta);
    thread::sleep(std::time::Duration::from_secs(1));

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test3.txt".into()]);
    command_line.set_options(vec!["-m".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test3.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    fs::remove_file("test3.txt").unwrap();
    assert_ne!(m, mb);
  }

  #[test]
  fn touch_file_date_time() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test4.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test4.txt").exists());
    let meta = fs::metadata("test4.txt").unwrap();
    let mb = FileTime::from_last_modification_time(&meta);

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test4.txt".into()]);
    command_line.set_options(vec!["-d".into(), "2018-01-01".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test4.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    fs::remove_file("test4.txt").unwrap();
    assert_ne!(m, mb);
  }

  #[test]
  fn touch_fiel_reffarence() {
    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test5.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    assert!(Path::new("test5.txt").exists());

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test6.txt".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test6.txt").unwrap();
    let mb = FileTime::from_last_modification_time(&meta);

    let mut command_line = CommandLine::new();
    command_line.set_name("touch");
    command_line.set_args(vec!["test5.txt".into(), "test6.txt".into()]);
    command_line.set_options(vec!["-r".into()]);
    let touch_commands = TouchCommands::new(command_line);
    match touch_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
    let meta = fs::metadata("test5.txt").unwrap();
    let m = FileTime::from_last_modification_time(&meta);
    fs::remove_file("test5.txt").unwrap();
    fs::remove_file("test6.txt").unwrap();

    assert_eq!(m.seconds(), mb.seconds());
  }
}