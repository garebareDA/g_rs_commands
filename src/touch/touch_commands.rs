use crate::parser::command_line::CommandLine;
use crate::parser::commands::Commands;

pub struct TouchCommands {
    command_line: CommandLine,
}

impl TouchCommands {
    pub fn new(command_line: CommandLine) -> TouchCommands {
        let mut command = TouchCommands {
            command_line: command_line,
        };
        return command;
    }
}

impl Commands for TouchCommands {
    fn help(&self) {
        println!("Usage: gtouch [OPTION]... FILE...");
    }

    fn version(&self) {
        println!("touch 0.1.0");
    }

    fn name(&self) -> &str {
        "gtouch"
    }

    fn run(&self) -> Result<(), String> {
        let command = &self.command_line;
        let args = command.get_args();
        let options = command.get_options();
        if args.len() == 1 {
            if self.print_help(options, &args[0]) {
                return Ok(());
            };
            if self.print_version(options, &args[0]) {
                return Ok(());
            };

            self.timestamp_update(&args[0])?;
            return Ok(());
        }
        return Ok(());
    }
}
