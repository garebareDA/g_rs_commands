use crate::parser::command_line::CommandLine;
use crate::parser::commands;

pub struct RmCommands {
    command_line: CommandLine,
    is_reverso: bool,
    is_interactive: bool,
}

impl RmCommands {
    pub fn new(command_line: CommandLine) -> RmCommands {
        let mut command = RmCommands {
            command_line: command_line,
            is_reverso: false,
            is_interactive: false,
        };

        for flag in command.command_line.get_options().iter() {
            match flag.as_str() {
                "-r" => command.is_reverso = true,
                "-i" => command.is_interactive = true,
                _ => (),
            }
        }

        return command;
    }

    pub fn get_interactive(&self) -> bool {
        return self.is_interactive;
    }
}

impl commands::Commands for RmCommands {
    fn help(&self) {
        println!("rm: remove files or directories");
    }

    fn version(&self) {
        println!("v0.1.0");
    }

    fn name(&self) -> &str {
        "grm"
    }

    fn run(&self) -> Result<(), String> {
        let command = &self.command_line;
        let args = command.get_args();
        let options = command.get_options();

        if args.len() > 1 && args.len() == 0 {
            println!("rm: missing operand");
            println!("Try 'grm --help' for more information.");
            return Ok(());
        }

        if self.print_help(options, &args[0]) {
            return Ok(());
        };

        if self.print_version(options, &args[0]) {
            return Ok(());
        };

        if self.is_reverso {
            self.remove_reverso(&args[0])?;
        } else {
            self.remove_file(&args[0])?;
        }

        Ok(())
    }
}
