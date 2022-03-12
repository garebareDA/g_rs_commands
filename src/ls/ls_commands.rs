use crate::parser::command_line;
use crate::parser::commands;

pub struct LsCommands {
  command_line: command_line::CommandLine,
}

impl LsCommands {
  pub fn new() -> LsCommands {
    LsCommands {
      command_line: command_line::CommandLine::new(),
    }
  }
}

impl commands::Commands for LsCommands {
  fn help(&self) {
    println!("help");
  }

  fn version(&self) {
    println!("v0.1.0");
  }

  fn name(&self) -> &str {
    "ls"
  }

  fn run(&self) {
    let command = &self.command_line;
    let args = command.get_args();
    let options = command.get_options();

    if args.len() != 2 && args.len() != 1 {
      println!("ls: missing operand");
      println!("Try 'ls --help' for more information.");
      return;
    }

    if self.print_help(options, &args[0]) { return; };
    if self.print_version(options, &args[0]) { return; };
    if args.len() == 1 {
      self.read_dir("./");
    } else {
      self.read_dir(&args[1]);
    }

  }
}
