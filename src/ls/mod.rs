pub mod ls_commands;
mod ls_file;

#[cfg(test)]
mod tests {
    use super::ls_commands;
    use crate::parser::command_line::CommandLine;
    use crate::parser::commands::Commands;

    #[test]
    fn ls_run() {
        let mut command_line = CommandLine::new();
        command_line.set_name("ls");
        command_line.set_args(vec!["./".into()]);
        command_line.set_options(vec![]);
        let ls_commands = ls_commands::LsCommands::new(command_line);
        match ls_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn ls_option() {
        let mut command_line = CommandLine::new();
        command_line.set_name("ls");
        command_line.set_args(vec!["./".into()]);
        command_line.set_options(vec!["-a".into(), "-l".into(), "-R".into(), "-r".into()]);
        let ls_commands = ls_commands::LsCommands::new(command_line);
        assert_eq!(ls_commands.get_is_show_hidden(), true);
        assert_eq!(ls_commands.get_is_show_details(), true);
        assert_eq!(ls_commands.get_is_show_reverso(), true);
        assert_eq!(ls_commands.get_is_reverse(), true);
        match ls_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn ls_help() {
        let mut command_line = CommandLine::new();
        command_line.set_name("ls");
        command_line.set_args(vec!["help".into()]);
        command_line.set_options(vec!["--help".into()]);
        let ls_commands = ls_commands::LsCommands::new(command_line);
        match ls_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn ls_version() {
        let mut command_line = CommandLine::new();
        command_line.set_name("ls");
        command_line.set_args(vec!["version".into()]);
        command_line.set_options(vec!["--version".into()]);
        let ls_commands = ls_commands::LsCommands::new(command_line);
        match ls_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn ls_no_args() {
        let mut command_line = CommandLine::new();
        command_line.set_name("ls");
        command_line.set_args(vec![]);
        command_line.set_options(vec![]);
        let ls_commands = ls_commands::LsCommands::new(command_line);
        match ls_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
    }
}
