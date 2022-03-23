use std::{path::Path, fs};

use super::touch_commands;
use filetime::{FileTime, set_file_times};
impl touch_commands::TouchCommands {
    pub(crate) fn timestamp_update<P:AsRef<Path> + Copy>(&self, path:P) -> Result<(), String> {
        match fs::metadata(&path) {
            Ok(metadata) => {
              set_file_times(path, FileTime::now(), FileTime::now()).ok().ok_or(format!("gtouch: {}: cannot set timestamp", &path.as_ref().display()))?;
            }
            Err(_) => {
                fs::File::create(&path).ok().ok_or("gtouch: cannot create file")?;
            }
        }
        return Ok(());
    }
}