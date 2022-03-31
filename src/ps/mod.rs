pub mod ps_commands;
pub mod processes;

#[cfg(test)]
mod tests {
  use super::ps_commands;
  use crate::parser::command_line::CommandLine;
  use crate::parser::commands::Commands;

  #[test]
  fn ps_run() {
    let mut command_line = CommandLine::new();
    command_line.set_name("ps");
    command_line.set_args(vec![]);
    command_line.set_options(vec![]);
    let ps_commands = ps_commands::PsCommands::new(command_line);
    match ps_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn ps_all_option() {
    let mut command_line = CommandLine::new();
    command_line.set_name("gps");
    command_line.set_args(vec![]);
    command_line.set_options(vec!["-e".into()]);
    let ps_commands = ps_commands::PsCommands::new(command_line);
    assert_eq!(ps_commands.get_is_show_all(), true);
    match ps_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn ps_terminal() {
    let mut command_line = CommandLine::new();
    command_line.set_name("ps");
    command_line.set_args(vec![]);
    command_line.set_options(vec!["-a".into()]);
    let ps_commands = ps_commands::PsCommands::new(command_line);
    assert_eq!(ps_commands.get_is_terminal(), true);
    match ps_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn ps_detail() {
    let mut command_line = CommandLine::new();
    command_line.set_name("ps");
    command_line.set_args(vec![]);
    command_line.set_options(vec!["-l".into()]);
    let ps_commands = ps_commands::PsCommands::new(command_line);
    assert_eq!(ps_commands.get_is_show_details(), true);
    match ps_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn ps_not_terminal() {
    let mut command_line = CommandLine::new();
    command_line.set_name("ps");
    command_line.set_args(vec![]);
    command_line.set_options(vec!["-x".into()]);
    let ps_commands = ps_commands::PsCommands::new(command_line);
    assert_eq!(ps_commands.get_is_not_terminal(), true);
    match ps_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }
}