use std::{fs, path::Path};
use chrono::{DateTime, Utc};

use super::touch_commands;
use filetime::{set_file_times, FileTime};
impl touch_commands::TouchCommands {
    pub(crate) fn timestamp_update<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        if path.as_ref().exists() {
            let (modified, accessed) = self.get_file_timestamp(&path)?;
            let mtime = if self.get_modification_time_udpate() {
                modified
            } else {
                FileTime::now()
            };

            let atime = if self.get_acsesss_time_update() {
                accessed
            } else {
                FileTime::now()
            };

            set_file_times(&path, atime, mtime).ok().ok_or(format!(
                "gtouch: {}: cannot set timestamp",
                &path.as_ref().display()
            ))?;
        } else {
            fs::File::create(&path)
                .ok()
                .ok_or("gtouch: cannot create file")?;
        }
        return Ok(());
    }

    pub(crate) fn reference_time<P: AsRef<Path>>(&self, args: Vec<P>) -> Result<(), String> {
        let (modified, accessed) = self.get_file_timestamp(&args[0])?;
        if args[1].as_ref().exists() {
            set_file_times(&args[1], accessed, modified)
                .ok()
                .ok_or(format!(
                    "gtouch: {}: cannot set timestamp",
                    &args[1].as_ref().display()
                ))?;
        } else {
            return Err(format!(
                "gtouch: {} is no such a file",
                args[1].as_ref().display()
            ));
        }
        return Ok(());
    }

    pub(crate) fn set_file_timestamp<P:AsRef<Path>>(&self, path: P, timestamp: &str) -> Result<(), String> {
      let date = DateTime::parse_from_rfc3339(timestamp).ok().ok_or("gtouch:data time prse error")?;
      let mtime = FileTime::from_unix_time(date.timestamp(), 0);
      let atime = FileTime::from_unix_time(date.timestamp(), 0);
      set_file_times(&path, atime, mtime).ok().ok_or(format!(
          "gtouch: {}: cannot set timestamp",
          &path.as_ref().display()
      ))?;
      return Ok(());
    }

    fn get_file_timestamp<P: AsRef<Path>>(&self, path: P) -> Result<(FileTime, FileTime), String> {
        let path = path.as_ref();
        let metadata = fs::metadata(path).ok().ok_or(format!(
            "gtouch: {}: No such file or directory",
            path.display()
        ))?;
        let modified = FileTime::from_last_modification_time(&metadata);
        let accessed = FileTime::from_last_access_time(&metadata);
        return Ok((modified, accessed));
    }
}
