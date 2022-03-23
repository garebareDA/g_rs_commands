pub mod which_commands;
pub mod which_find;

#[cfg(test)]
mod tests {
  use super::which_commands;
  use crate::parser::commands::Commands;
  use crate::parser::command_line::CommandLine;

  #[test]
  fn which_no_option() {
    let mut command_line = CommandLine::new();
    command_line.set_name("which");
    command_line.set_args(vec!["ls".into()]);
    command_line.set_options(vec![]);
    let which_commands = which_commands::WhichCommands::new(command_line);
    match which_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }

  #[test]
  fn which_a() {
    let mut command_line = CommandLine::new();
    command_line.set_name("which");
    command_line.set_args(vec!["ls".into()]);
    command_line.set_options(vec!["-a".into()]);
    let which_commands = which_commands::WhichCommands::new(command_line);
    match which_commands.run() {
      Ok(_) => {}
      Err(e) => panic!("{}", e),
    }
  }
}