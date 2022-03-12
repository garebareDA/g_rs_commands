use crate::parser::command_line;
use crate::parser::commands;
use super::ls_file::File;

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

    if args.len() != 1 && args.len() != 0 {
      println!("ls: missing operand");
      println!("Try 'ls --help' for more information.");
      return;
    }

    if args.len() == 1 {
      if self.print_help(options, &args[0]) {
        return;
      };

      if self.print_version(options, &args[0]) {
        return;
      };
    }

    let dir:Vec<File>;
    if args.len() == 0 {
      dir = self.read_dir("./");
    } else {
      dir = self.read_dir(&args[0]);
    }

    println!("{}", dir[0].get_modification_time_string());
  }
}
