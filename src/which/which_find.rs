use std::{
    env,
    path::{Path, PathBuf},
};

use super::which_commands::WhichCommands;

impl WhichCommands {
    pub(crate) fn find_commands<P: AsRef<Path>>(&self, path: P) -> Option<Vec<PathBuf>> {
        env::var_os("PATH").and_then(|paths| {
            let paths = env::split_paths(&paths).filter_map(|dir| {
                let full_path = dir.join(&path);
                if full_path.exists() && full_path.is_file() {
                    return Some(full_path);
                }
                return None;
            });
            return Some(paths.collect());
        })
    }
}
