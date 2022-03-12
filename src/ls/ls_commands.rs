use crate::file::file::File;
use crate::parser::command_line;
use crate::parser::commands;
use colored::*;

pub struct LsCommands {
  command_line: command_line::CommandLine,
}

impl LsCommands {
  pub fn new() -> LsCommands {
    LsCommands {
      command_line: command_line::CommandLine::new(),
    }
  }

  fn print_file_name(&self, file: &File) {
    if file.is_dir() {
      println!("{}  ", file.get_name().blue());
    } else {
      println!("{}  ", file.get_name());
    }
  }

  fn print_file_details(&self, file: &File) {
    println!(
      "{0} {1: >2} {2: >8} {3: >8} {4: >4} {5} {6}",
      file.get_permission_string(),
      file.get_hard_link_count(),
      file.get_user(),
      file.get_group(),
      file.get_size(),
      file.get_modification_time(),
      if file.is_dir() {
        file.get_name().blue()
      } else {
        file.get_name().white()
      },
    );
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

  fn run(&self) -> Result<(), String> {
    let command = &self.command_line;
    let args = command.get_args();
    let options = command.get_options();
    let mut is_show_hidden = false;
    let mut is_show_details = false;

    if args.len() != 1 && args.len() != 0 {
      println!("ls: missing operand");
      println!("Try 'ls --help' for more information.");
      return Ok(());
    }

    if args.len() == 1 {
      if self.print_help(options, &args[0]) {
        return Ok(());
      };

      if self.print_version(options, &args[0]) {
        return Ok(());
      };
    }

    let mut dir: Vec<File>;
    if args.len() == 0 {
      dir = self.read_dir("./")?;
    } else {
      dir = self.read_dir(&args[0])?;
    }

    for option in options {
      if option == "-a" {
        is_show_hidden = true;
      }

      if option == "-l" {
        is_show_details = true;
      }
    }

    for file in dir {
      if file.is_hidden() && !is_show_hidden {
        continue;
      }

      if is_show_details {
        self.print_file_details(&file);
      } else {
        self.print_file_name(&file);
      }
    }

    return Ok(());
  }
}
