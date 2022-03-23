extern crate g_rs_commands;
use std::process;
use g_rs_commands::cat::gcat_commands::CatCommands;
use g_rs_commands::parser::commands::Commands;
use g_rs_commands::parser::command_line::CommandLine;

fn main() {
  let rm_commands = CatCommands::new(CommandLine::new());
  match rm_commands.run() {
    Ok(_) => {},
    Err(e) => {
        eprintln!("{}", e);
        process::exit(1);
    },
  }
}