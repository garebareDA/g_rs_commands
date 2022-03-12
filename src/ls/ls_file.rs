use super::ls_commands::LsCommands;
use std::os::unix::prelude::MetadataExt;
use std::path::Path;
use std::{fs, os::unix::prelude::PermissionsExt};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub(crate) struct File {
  name: String,
  group: String,
  user: String,
  mode: u32,
  permission: u32,
  modification_time: i64,
  is_hidden: bool,
  is_dir: bool,
}

impl File {
  pub fn new(
    name: &str,
    group: &str,
    user: &str,
    mode: u32,
    permission: u32,
    modification_time: i64,
    is_hidden: bool,
    is_dir: bool,
  ) -> File {
    File {
      name: name.to_string(),
      group: group.to_string(),
      user: user.to_string(),
      mode: mode,
      permission: permission,
      modification_time: modification_time,
      is_hidden: is_hidden,
      is_dir: is_dir,
    }
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }

  pub fn is_dir(&self) -> bool {
    self.is_dir
  }
}

impl LsCommands {
  pub(crate) fn read_dir<P: AsRef<Path>>(&self, path: P) -> Vec<File> {
    let files = fs::read_dir(path)
      .unwrap()
      .map(|entry| {
        let entry = entry.unwrap();
        let user = get_user_by_uid(entry.metadata().unwrap().uid()).unwrap();
        let group = get_group_by_gid(entry.metadata().unwrap().gid()).unwrap();
        let mode = entry.metadata().unwrap().mode();
        let permission = entry.metadata().unwrap().permissions().mode();
        let name = entry.file_name().into_string().unwrap();
        let is_dir = entry.file_type().unwrap().is_dir();
        let modification_time = entry.metadata().unwrap().mtime();
        let is_hidden = name.starts_with(".");

        return File::new(
          &name,
          &group.name().to_os_string().into_string().unwrap(),
          &user.name().to_os_string().into_string().unwrap(),
          mode,
          permission,
          modification_time,
          is_hidden,
          is_dir,
        );
      })
      .collect();

    println!("{:?}", files);
    return files;
  }
}
