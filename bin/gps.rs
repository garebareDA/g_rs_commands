extern crate g_rs_commands;
use g_rs_commands::ps::ps_commands::PsCommands;
use g_rs_commands::parser::commands::Commands;
use g_rs_commands::parser::command_line::CommandLine;
use std::process;
fn main() {
    let ps_commands = PsCommands::new(CommandLine::new());
    match ps_commands.run() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        },
    }
}