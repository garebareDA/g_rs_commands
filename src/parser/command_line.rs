use std::env;

pub struct CommandLine {
  args: Vec<String>,
  options: Vec<String>,
}

impl CommandLine {
  pub fn new() -> CommandLine {
    let args:Vec<String> = env::args().collect();
    let options = args.iter().filter(|arg| arg.starts_with("-"))
      .map(|arg| arg.to_string())
      .collect();
    let args = args.iter().filter(|arg| !arg.starts_with("-"))
      .map(|arg| arg.to_string())
      .collect();

    CommandLine {
      args: args,
      options: options,
    }
  }

  pub fn get_args(&self) -> &Vec<String> {
    &self.args
  }

  pub fn get_options(&self) -> &Vec<String> {
    &self.options
  }
}
