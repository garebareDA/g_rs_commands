use crate::parser::command_line::CommandLine;
use crate::parser::commands::Commands;

pub struct TouchCommands {
    command_line: CommandLine,
    access_time_update: bool,
    modification_time_update: bool,
    is_date_time: bool,
    reference_time: bool,
}

impl TouchCommands {
    pub fn new(command_line: CommandLine) -> TouchCommands {
        let mut command = TouchCommands {
            command_line: command_line,
            access_time_update: false,
            modification_time_update: false,
            is_date_time: false,
            reference_time: false,
        };

        for flag in command.command_line.get_options().iter() {
            match flag.as_str() {
                "-a" => command.access_time_update = true,
                "-m" => command.modification_time_update = true,
                "-d" => command.is_date_time = true,
                "-r" => command.reference_time = true,
                _ => (),
            }
        }

        return command;
    }

    pub(crate) fn get_access_time_update(&self) -> bool {
        return self.access_time_update;
    }

    pub(crate) fn get_modification_time_update(&self) -> bool {
        return self.modification_time_update;
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
        } else if args.len() == 2 && self.is_date_time {
            self.set_file_timestamp(&args[1], &args[0])?;
            return Ok(());
        } else if args.len() == 2 && self.reference_time {
            self.reference_time(args.to_vec())?;
            return Ok(());
        }

        return Err(String::from("gtouch: missing operand"));
    }
}
