extern crate g_rs_commands;
use std::process;
use g_rs_commands::rm::rm_commands;
use g_rs_commands::parser::command_line::CommandLine;
use g_rs_commands::parser::commands::Commands;

fn main() {
  let rm_commands = rm_commands::RmCommands::new(CommandLine::new());
  match rm_commands.run() {
    Ok(_) => {},
    Err(e) => {
        eprintln!("{}", e);
        process::exit(1);
    },
  }
}
