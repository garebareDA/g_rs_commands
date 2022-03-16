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
    let name = file.get_name();
    if file.get_meta().is_some() {
      let meta = file.get_meta().unwrap();
      if meta.is_dir() {
        println!("{}", name.green());
        return;
      }
    }
    println!("{}", name.white());
  }

  fn print_file_details(&self, file: &File) {
    let meta = file.get_meta();
    let name = file.get_name();
    let mut name_color = name.white();
    if meta.is_some() {
      let meta = meta.unwrap();
      let user = meta.get_user();
      let group = meta.get_group();
      let permission = meta.get_permission_string();
      let data_time = meta.get_modification_time();
      let hard_link = meta.get_hard_link_count();
      let byte_size = meta.get_size();
      if meta.is_dir() {
        name_color = name.blue();
      }

      let symlink = match meta.get_sym_link() {
        Some(s) => format!(" -> {}", s),
        None => "".to_string(),
      };
      let symlink_color = if meta.is_symlink_dir() {
        symlink.blue()
      } else {
        symlink.white()
      };

      println!("{0} {1: >8} {2: >8} {3: >8} {4: >8} {5} {6: <8} {7: <8}",
        permission,
        hard_link,
        user,
        group,
        byte_size,
        data_time,
        name_color,
        symlink_color,
      );
    } else {
      println!("{}", name_color);
    }
  }

  fn show_reverso (&self, file:&File) {
    if file.get_meta().is_some() != true {
      return;
    }
    let name = file.get_name();
    let dirs = self.read_dir(name);
    println!("{:?}", dirs);
  }

  fn its_show_time (&self) {
    println!("{}", "its show time by tafumi");
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
    "gls"
  }

  fn run(&self) -> Result<(), String> {
    let command = &self.command_line;
    let args = command.get_args();
    let options = command.get_options();
    let mut is_show_hidden = false;
    let mut is_show_details = false;
    let mut is_show_reverso = false;

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

      if args[0] == "its_show_time" {
        self.its_show_time();
        return Ok(());
      }
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

      if option == "-R" {
        is_show_reverso = true;
      }

      if option == "-r" {
        dir.reverse();
      }
    }

    let mut dirs:Vec<&File> = Vec::new();
    for file in dir.iter() {
      if file.get_name().starts_with(".") && !is_show_hidden {
          continue;
      }

      if is_show_details {
        self.print_file_details(&file);
      } else {
        self.print_file_name(&file);
      }

      if is_show_reverso && file.get_meta().is_some() {
        let meta = file.get_meta().unwrap();
        if meta.is_dir() {
          dirs.push(file);
        }
      }
    }

    if is_show_reverso {
      self.show_reverso(&dirs[0]);
    }

    return Ok(());
  }
}
