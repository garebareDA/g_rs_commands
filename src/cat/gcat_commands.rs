use crate::parser::command_line::CommandLine;
use crate::parser::commands;

pub struct CatCommands {
  command_line: CommandLine,
}

impl CatCommands {
  pub fn new(command_line: CommandLine) -> CatCommands {
    return CatCommands { command_line: command_line };
  }
}
impl commands::Commands for CatCommands {
  fn help(&self) {
    println!("gcat: concatenate files and print on the standard output");
    println!("usage: cat [OPTION]... [FILE]...");
    println!("");
    println!("Concatenate FILE(s), or standard input, to standard output.");
    println!("");
    println!("  -A, --show-all           equivalent to -vET");
    println!("  -b, --number-nonblank    number nonempty output lines, overrides -n");
    println!("  -h, --help               display this help and exit");
    println!("  -V, --version            output version information and exit");
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