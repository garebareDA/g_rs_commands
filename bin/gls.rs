extern crate g_rs_commands;
use g_rs_commands::ls;
use g_rs_commands::parser::commands::Commands;

fn main () {
  let ls_commands = ls::ls_commands::LsCommands::new();
  ls_commands.run();
}