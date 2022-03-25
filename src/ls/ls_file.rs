use super::ls_commands::LsCommands;
use crate::file::file::File;
use crate::file::meta::Meta;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use users::{get_group_by_gid, get_user_by_uid};

impl LsCommands {
    pub(crate) fn read_dir<P: AsRef<Path> + Copy>(&self, path: P) -> Result<Vec<File>, String> {
        let files = fs::read_dir(path)
            .ok()
            .ok_or(format!(
                "gls: cannot access '{}' : No such a file or directory",
                path.as_ref().display()
            ))?
            .map(|entry| self.insert_files(entry))
            .collect();
        return files;
    }

    fn insert_files(&self, entry: Result<DirEntry, Error>) -> Result<File, String> {
        let entry = entry.ok().ok_or(String::new())?;
        let metadata = entry.metadata();
        let name = entry.file_name().to_os_string();

        if metadata.is_ok() {
            let metadata = metadata.ok().ok_or(String::new())?;
            let group = get_group_by_gid(metadata.gid());
            let user = get_user_by_uid(metadata.uid());
            let permission = metadata.permissions().mode();
            let data_time = metadata.modified().ok();
            let hard_link = metadata.nlink();
            let byte_size = metadata.len();
            let file_type = entry.file_type().ok();
            let symlink = if metadata.is_symlink() {
                entry.path().read_link().ok()
            } else {
                None
            };
            let meta = Meta::new(
                group, user, permission, data_time, hard_link, byte_size, file_type, symlink,
            );

            return Ok(File::new(name, Some(meta)));
        }

        return Ok(File::new(name, None));
    }
}
