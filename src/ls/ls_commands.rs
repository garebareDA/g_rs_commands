use crate::file::file::File;
use crate::parser::command_line::CommandLine;
use crate::parser::commands;
use colored::*;

pub struct LsCommands {
  command_line: CommandLine,
  is_show_hidden: bool,
  is_show_details: bool,
  is_show_reverso: bool,
  is_reverse: bool,
}

impl LsCommands {
  pub fn new(command_line:CommandLine) -> LsCommands {
    let mut command = LsCommands {
      command_line: command_line,
      is_show_hidden: false,
      is_show_details: false,
      is_show_reverso: false,
      is_reverse: false,
    };

    let options = command.command_line.get_options();
    for option in options.iter() {
      if option == "-a" {
        command.is_show_hidden = true;
      }

      if option == "-l" {
        command.is_show_details = true;
      }

      if option == "-R" {
        command.is_show_reverso = true;
      }

      if option == "-r" {
        command.is_reverse = true;
      }
    }

    return command;
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

      println!(
        "{0} {1: >8} {2: >8} {3: >8} {4: >8} {5} {6: <8} {7: <8}",
        permission, hard_link, user, group, byte_size, data_time, name_color, symlink_color,
      );
    } else {
      println!("{}", name_color);
    }
  }

  fn show_reverso(&self, mut file: Vec<&File>, path: &str) -> Result<(), String> {
    if self.is_reverse {
      file.reverse()
    }

    for f in file {
      if f.get_meta().is_some() == false {
        continue;
      }
      let mut files: Vec<&File> = Vec::new();
      let name = f.get_name();
      let mut dirs = self.read_dir(&format!("{}/{}", path, &name))?;
      if self.is_reverse {
        dirs.reverse();
      }

      println!("\n {}", format!("{}/{}", path, &name).blue());
      for dir in dirs.iter() {
        let dir_meta = dir.get_meta().unwrap();
        if dir_meta.is_dir() {
          files.push(dir);
        }

        if self.is_show_details {
          self.print_file_details(dir);
        } else {
          self.print_file_name(dir);
        }
      }
      self.show_reverso(files, &format!("{}/{}", path, &name))?;
    }
    Ok(())
  }

  pub fn get_is_show_hidden(&self) -> bool {
    return self.is_show_hidden;
  }

  pub fn get_is_show_details(&self) -> bool {
    return self.is_show_details;
  }

  pub fn get_is_show_reverso(&self) -> bool {
    return self.is_show_reverso;
  }

  pub fn get_is_reverse(&self) -> bool {
    return self.is_reverse;
  }

  fn its_show_time(&self) {
    println!("{}", "its show time by tafumi");
  }
}

impl commands::Commands for LsCommands {
  fn help(&self) {
    println!("Usage: ls [OPTION]... [FILE]...");
    println!("List information about the FILEs.");
    println!("");
    println!("  -a,  do not ignore entries starting with .");
    println!("  -l,  print in long format");
    println!("  -R,  list subdirectories recursively");
    println!("  -r,  reverse order while sorting");
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

    if args.len() != 1 && args.len() != 0 {
      println!("ls: missing operand");
      println!("Try 'gls --help' for more information.");
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

    if self.is_reverse {
      dir.reverse()
    }

    let mut dirs: Vec<&File> = Vec::new();
    for file in dir.iter() {
      if file.get_name().starts_with(".") && !self.is_show_hidden {
        continue;
      }

      if self.is_show_details {
        self.print_file_details(&file);
      } else {
        self.print_file_name(&file);
      }

      if self.is_show_reverso && file.get_meta().is_some() {
        let meta = file.get_meta().unwrap();
        if meta.is_dir() {
          dirs.push(file);
        }
      }
    }

    if self.is_show_reverso {
      self.show_reverso(dirs, ".")?;
    }

    return Ok(());
  }
}
