extern crate g_rs_commands;
use std::process;

use g_rs_commands::which::which_commands;
use g_rs_commands::parser::command_line::CommandLine;
use g_rs_commands::parser::commands::Commands;

fn main() {
  let which_commands = which_commands::WhichCommands::new(CommandLine::new());
  match which_commands.run() {
    Ok(_) => {},
    Err(e) => {
        eprintln!("{}", e);
        process::exit(1);
    },
  }
}