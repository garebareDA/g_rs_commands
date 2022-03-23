use std::{fs, path::Path};

use super::touch_commands;
use filetime::{set_file_times, FileTime};
impl touch_commands::TouchCommands {
    pub(crate) fn timestamp_update<P: AsRef<Path> + Copy>(&self, path: P) -> Result<(), String> {
        match fs::metadata(&path) {
            Ok(metadata) => {
                let mtime = if self.get_modification_time_udpate() {
                    FileTime::from_last_modification_time(&metadata)
                } else {
                    FileTime::now()
                };

                let atime = if self.get_acsesss_time_update() {
                    FileTime::from_last_access_time(&metadata)
                } else {
                    FileTime::now()
                };
                set_file_times(path, atime, mtime)
                    .ok()
                    .ok_or(format!(
                        "gtouch: {}: cannot set timestamp",
                        &path.as_ref().display()
                    ))?;
            }
            Err(_) => {
                fs::File::create(&path)
                    .ok()
                    .ok_or("gtouch: cannot create file")?;
            }
        }
        return Ok(());
    }
}
