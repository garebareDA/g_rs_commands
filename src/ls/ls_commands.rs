use crate::parser::command_line;

pub fn ls_command () {
  let command = command_line::Command::new();
  let args = command.get_args();
  let options = command.get_options();

  if args.len() != 1 {
    println!("ls: missing operand");
    println!("Try 'ls --help' for more information.");
  }

  
}