extern crate g_rs_commands;
use g_rs_commands::ls;
use g_rs_commands::parser::commands::Commands;
use g_rs_commands::parser::command_line::CommandLine;

fn main () {
  let ls_commands = ls::ls_commands::LsCommands::new(CommandLine::new());
  match ls_commands.run() {
    Ok(_) => {},
    Err(e) => println!("{}", e),
  }
}