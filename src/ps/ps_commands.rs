use crate::parser::command_line::CommandLine;
use crate::parser::commands;

pub struct PsCommands {
    command_line: CommandLine,
    is_show_all: bool,
}

impl PsCommands {
    pub fn new(command_line: CommandLine) -> PsCommands {
        let commands = PsCommands {
            command_line,
            is_show_all: false,
        };

        for flag in commands.command_line.get_options().iter() {
            match flag.as_str() {
                "-a" => commands.is_show_all = true,
                _ => (),
            }
        }

        return commands;
    }

    pub fn get_is_show_all(&self) -> bool {
        return self.is_show_all;
    }
}

impl commands::Commands for PsCommands {
    fn help(&self) {
        println!("gps: Print a list of processes to stdout.");
        println!("usage: ps [OPTION]...");
        println!("");
        println!("  -o, --sort=STRING   sort by STRING");
        println!("  -r, --reverse       reverse order while sorting");
        println!("  -v, --version       print version information and exit");
        println!("  -h, --help          display this help and exit");
    }

    fn version(&self) {
        println!("gps 0.1.0");
    }

    fn name(&self) -> &str {
        "gps"
    }

    fn run(&self) -> Result<(), String> {
        let command = &self.command_line;
        let args = command.get_args();

        if args.len() != 0 {
            println!("ps: missing operand");
            println!("Try 'gps --help' for more information.");
            return Ok(());
        }
        self.processes_display()?;
        return Ok(());
    }
}
