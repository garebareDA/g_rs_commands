use std::env;

pub struct CommandLine {
  command: String,
  args: Vec<String>,
  options: Vec<String>,
}

impl CommandLine {
  pub fn new() -> CommandLine {
    let mut args:Vec<String> = env::args().collect();
    let command = args.remove(0);
    let options = args.iter().filter(|arg| arg.starts_with("-"))
      .map(|arg| arg.to_string())
      .collect();
    let args = args.iter().filter(|arg| !arg.starts_with("-"))
      .map(|arg| arg.to_string())
      .collect();

    CommandLine {
      command: command,
      args: args,
      options: options,
    }
  }

  pub fn get_args(&self) -> &Vec<String> {
    &self.args
  }

  pub fn set_args(&mut self, args: Vec<String>) {
    self.args = args;
  }

  pub fn get_options(&self) -> &Vec<String> {
    &self.options
  }

  pub fn set_options(&mut self, options: Vec<String>) {
    self.options = options;
  }

  pub fn get_name(&self) -> &str {
    &self.command
  }

  pub fn set_name(&mut self, name: &str) {
    self.command = name.into();
  }
}
