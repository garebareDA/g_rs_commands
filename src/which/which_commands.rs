use crate::parser::command_line::CommandLine;
use crate::parser::commands;

pub struct WhichCommands {
  command_line: CommandLine,
  is_all: bool,
}

impl WhichCommands {
  pub fn new(command_line: CommandLine) -> WhichCommands {
    let mut command = WhichCommands {
      command_line: command_line,
      is_all: false,
    };

    for flag in command.command_line.get_options().iter() {
      match flag.as_str() {
        "-a" => command.is_all = true,
        _ => (),
      }
    }

    return command;
  }
}

impl commands::Commands for WhichCommands {
  fn help(&self) {
    println!("Usage: witch [OPTION]... [STRING]...");
  }

  fn version(&self) {
    println!("witch 0.1.0");
  }

  fn name(&self) -> &str {
    "gwitch"
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

      let result = self.find_commands(&args[0]).ok_or("Not found PATH")?;
      if self.is_all {
        for path in result.iter() {
          println!("{}", path.display());
        }
        return Ok(());
      }

      println!("{}", result[0].display());
    } else {
      println!("witch: missing operand");
      println!("Try 'gwitch --help' for more information.");
    }

    return Ok(());
  }
}
