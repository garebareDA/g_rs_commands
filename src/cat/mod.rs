pub mod gcat_commands;
pub mod gcat_file;

#[cfg(test)]
mod tests {
    use super::gcat_commands;
    use crate::parser::command_line::CommandLine;
    use crate::parser::commands::Commands;
    use std::fs::File;
    #[test]
    fn cat_command() {
        let mut command_line = CommandLine::new();
        command_line.set_args(vec!["test.txt".into()]);
        File::create("test.txt").unwrap();
        command_line.set_options(vec!["-n".into(), "-e".into(), "-b".into()]);
        let cat_commands = gcat_commands::CatCommands::new(command_line);
        assert_eq!(cat_commands.get_show_number(), true);
        assert_eq!(cat_commands.get_show_end(), true);
        assert_eq!(cat_commands.get_hide_blank(), true);
        match cat_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }
}
