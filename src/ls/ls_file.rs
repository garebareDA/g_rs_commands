use super::ls_commands::LsCommands;
use std::os::unix::prelude::MetadataExt;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, os::unix::prelude::PermissionsExt};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug)]
pub(crate) struct File {
  name: String,
  group: String,
  user: String,
  permission: u32,
  modification_time: SystemTime,
  hard_link_count: u64,
  byte_size: u64,
  is_hidden: bool,
  is_dir: bool,
}

impl File {
  pub fn new(
    name: &str,
    group: &str,
    user: &str,
    permission: u32,
    modification_time: SystemTime,
    hard_link_count: u64,
    byte_size: u64,
    is_hidden: bool,
    is_dir: bool,
  ) -> File {
    File {
      name: name.into(),
      group: group.into(),
      user: user.into(),
      permission: permission,
      modification_time: modification_time,
      hard_link_count: hard_link_count,
      byte_size: byte_size,
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

  pub fn is_hidden(&self) -> bool {
    self.is_hidden
  }

  pub fn get_permission(&self) -> u32 {
    self.permission
  }

  pub fn get_modification_time(&self) -> SystemTime {
    self.modification_time
  }

  pub fn get_user(&self) -> &str {
    &self.user
  }

  pub fn get_group(&self) -> &str {
    &self.group
  }

  pub fn get_hard_link_count(&self) -> u64 {
    self.hard_link_count
  }

  pub fn get_byte_size(&self) -> u64 {
    self.byte_size
  }

  pub fn get_size(&self) -> String {
    let size = self.get_byte_size();
    if size < 1024 {
      return format!("{}", size);
    }
    if size < 1024 * 1024 {
      return format!("{}K", size / 1024);
    }
    if size < 1024 * 1024 * 1024 {
      return format!("{}M", size / 1024 / 1024);
    }
    format!("{}G", size / 1024 / 1024 / 1024)
  }

  pub fn get_modification_time_string(&self) -> String {
    return format!(
      "{}",
      self.get_modification_time().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    );
  }

  fn get_permission_string(&self) -> String {
    let permission = self.get_permission();
    let mut permission_string = String::new();
    if permission & 0o400 != 0 {
      permission_string.push('r');
    } else {
      permission_string.push('-');
    }
    if permission & 0o200 != 0 {
      permission_string.push('w');
    } else {
      permission_string.push('-');
    }
    if permission & 0o100 != 0 {
      permission_string.push('x');
    } else {
      permission_string.push('-');
    }
    permission_string.push('/');

    if permission & 0o040 != 0 {
      permission_string.push('r');
    } else {
      permission_string.push('-');
    }
    if permission & 0o020 != 0 {
      permission_string.push('w');
    } else {
      permission_string.push('-');
    }
    if permission & 0o010 != 0 {
      permission_string.push('x');
    } else {
      permission_string.push('-');
    }
    permission_string.push('/');

    if permission & 0o004 != 0 {
      permission_string.push('r');
    } else {
      permission_string.push('-');
    }
    if permission & 0o002 != 0 {
      permission_string.push('w');
    } else {
      permission_string.push('-');
    }
    if permission & 0o001 != 0 {
      permission_string.push('x');
    } else {
      permission_string.push('-');
    }

    return permission_string;
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
        let permission = entry.metadata().unwrap().permissions().mode();
        let name = entry.file_name().into_string().unwrap();
        let hard_link = entry.metadata().unwrap().nlink();
        let byte_size = entry.metadata().unwrap().len();
        let is_dir = entry.file_type().unwrap().is_dir();
        let modification_time = entry.metadata().unwrap().modified().unwrap();
        let is_hidden = name.starts_with(".");

        return File::new(
          &name,
          &group.name().to_os_string().into_string().unwrap(),
          &user.name().to_os_string().into_string().unwrap(),
          permission,
          modification_time,
          hard_link,
          byte_size,
          is_hidden,
          is_dir,
        );
      })
      .collect();

    return files;
  }
}
