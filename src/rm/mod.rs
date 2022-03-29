pub mod rm_commands;
pub mod rm_file;

#[cfg(test)]
pub mod tests {
    use crate::parser::command_line::CommandLine;
    use crate::parser::commands::Commands;
    use crate::rm::rm_commands::RmCommands;
    use std::fs;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn rm_file() {
        File::create("test.txt").unwrap();
        let mut command_line = CommandLine::new();
        command_line.set_name("rm");
        command_line.set_args(vec!["test.txt".into()]);
        command_line.set_options(vec![]);
        let rm_commands = RmCommands::new(command_line);
        match rm_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
        assert!(!Path::new("test.txt").exists());
    }

    #[test]
    fn rm_dir() {
        fs::create_dir("test_dir").unwrap();
        let mut command_line = CommandLine::new();
        command_line.set_name("rm");
        command_line.set_args(vec!["test_dir".into()]);
        command_line.set_options(vec![]);
        let rm_commands = RmCommands::new(command_line);
        match rm_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
        assert!(!Path::new("test_dir").exists());
    }

    #[test]
    fn rm_reverso() {
        fs::create_dir_all("test_dir2/test").unwrap();
        File::create("test_dir2/test/test.txt").unwrap();
        File::create("test_dir2/test2.txt").unwrap();

        let mut command_line = CommandLine::new();
        command_line.set_name("rm");
        command_line.set_args(vec!["test_dir2".into()]);
        command_line.set_options(vec!["-r".into()]);
        let rm_commands = RmCommands::new(command_line);
        match rm_commands.run() {
            Ok(_) => {}
            Err(e) => panic!("{}", e),
        }
        assert!(!Path::new("test_dir2").exists());
    }

    #[test]
    fn rm_interactive() {
        let mut command_line = CommandLine::new();
        command_line.set_name("rm");
        command_line.set_args(vec!["test_dir3".into()]);
        command_line.set_options(vec!["-r".into(), "-i".into()]);
        let rm_commands = RmCommands::new(command_line);
        assert_eq!(rm_commands.get_interactive(), true);
    }
}
