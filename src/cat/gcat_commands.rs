use crate::parser::command_line::CommandLine;
use crate::parser::commands;

pub struct CatCommands {
    command_line: CommandLine,
    show_end: bool,
    show_number: bool,
    hide_blank: bool,
}

impl CatCommands {
    pub fn new(command_line: CommandLine) -> CatCommands {
        let mut command = CatCommands {
            command_line,
            show_end: false,
            show_number: false,
            hide_blank: false,
        };

        for flag in command.command_line.get_options().iter() {
            match flag.as_str() {
                "-n" => command.show_number = true,
                "-e" => command.show_end = true,
                "-b" => command.hide_blank = true,
                _ => (),
            }
        }

        return command;
    }

    pub(crate) fn get_show_end(&self) -> bool {
        self.show_end
    }

    pub(crate) fn get_show_number(&self) -> bool {
        self.show_number
    }

    pub(crate) fn get_hide_blank(&self) -> bool {
        self.hide_blank
    }
}
impl commands::Commands for CatCommands {
    fn help(&self) {
        println!("gcat: concatenate files and print on the standard output");
        println!("usage: cat [OPTION]... [FILE]...");
        println!("");
        println!("Concatenate FILE(s), or standard input, to standard output.");
        println!("");
        println!("  -e,   display $ at end of each line");
        println!("  -n,   number all output lines");
        println!("  -b,   number nonempty output lines, overrides -n");
        println!("  -h, --help               display this help and exit");
        println!("  -v, --version            output version information and exit");
    }

    fn version(&self) {
        println!("v0.1.0");
    }

    fn name(&self) -> &str {
        "gcat"
    }

    fn run(&self) -> Result<(), String> {
        let command = &self.command_line;
        let args = command.get_args();
        let options = command.get_options();
        if args.len() < 1 {
            println!("gcat: missing operand");
            println!("Try 'gcat --help' for more information.");
            return Ok(());
        }

        if args.len() == 1 {
            if self.print_help(options, &args[0]) {
                return Ok(());
            }

            if self.print_version(options, &args[0]) {
                return Ok(());
            }
        }
        self.print_files(&args)?;
        return Ok(());
    }
}
