pub trait Commands {
  fn help(&self);
  fn version(&self);
  fn name(&self) -> &str;
  fn run(&self) -> Result<(), String>;
  fn print_help(&self, options: &Vec<String>, arg: &str) -> bool{
    if options.contains(&"--version".to_string())
      || options.contains(&"-v".to_string())
      || arg == "version"
    {
      self.version();
      return true;
    }
    false
  }
  fn print_version(&self, options: &Vec<String>, arg: &str) -> bool {
    if options.contains(&"--help".to_string())
      || options.contains(&"-h".to_string())
      || arg == "help"
    {
      self.help();
      return  true;
    }
    false
  }
}
