use chrono::{DateTime, Utc};
use std::fs::FileType;
use std::{path::PathBuf, time::SystemTime};
use users::Group;

#[derive(Debug)]
pub struct Meta {
  group: Option<Group>,
  user: Option<users::User>,
  permission: u32,
  modification_time: Option<SystemTime>,
  hard_link_count: u64,
  byte_size: u64,
  file_type: Option<FileType>,
  sym_link: Option<PathBuf>,
}

impl Meta {
  pub fn new(
    group: Option<Group>,
    user: Option<users::User>,
    permission: u32,
    modification_time: Option<SystemTime>,
    hard_link_count: u64,
    byte_size: u64,
    file_type: Option<FileType>,
    sym_link: Option<PathBuf>,
  ) -> Meta {
    Meta {
      group,
      user,
      permission,
      modification_time,
      hard_link_count,
      byte_size,
      file_type,
      sym_link,
    }
  }

  pub fn get_hard_link_count(&self) -> u64 {
    self.hard_link_count
  }

  pub fn get_user(&self) -> &str {
    match self.user {
      Some(ref user) => {
        return user.name().to_str().unwrap_or("<unknown>");
      }
      None => "<unknown>",
    }
  }

  pub fn get_group(&self) -> &str {
    match self.group {
      Some(ref group) => {
        return group.name().to_str().unwrap_or("<unknown>");
      }
      None => "<unknown>",
    }
  }

  pub fn is_dir(&self) -> bool {
    match self.file_type {
      Some(t) => t.is_dir(),
      _ => false,
    }
  }

  pub fn get_byte_size(&self) -> u64 {
    return self.byte_size;
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

  pub fn get_permission(&self) -> u32 {
    return self.permission;
  }

  pub fn get_modification_time(&self) -> String {
    match self.modification_time {
      Some(time) => {
        let date_time: DateTime<Utc> = time.into();
        let formatted = date_time.format("%Y-%m-%d %H:%M:%S").to_string();
        return formatted;
      }
      None => "<unknown>".to_string(),
    }
  }

  pub fn get_permission_string(&self) -> String {
    let permission = self.get_permission();
    let mut permission_string = String::new();
    if self.is_dir() {
      permission_string.push('d');
    } else {
      permission_string.push('-');
    }

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

  pub fn get_sym_link(&self) -> Option<&str> {
    match self.sym_link {
      Some(ref path) => {
        path.to_str()
      },
      None => (None),
    }
  }

  pub fn is_symlink_dir(&self) -> bool {
    match &self.sym_link {
      Some(t) => t.is_dir(),
      _ => false,
    }
  }
}
