use std::ffi::OsString;
use super::meta::Meta;

#[derive(Debug)]
pub(crate) struct File {
  name: OsString,
  meta:Option<Meta>,
}

impl File {
  pub fn new(
    name: OsString,
    meta: Option<Meta>,
  ) -> File {
    File {
      name,
      meta,
    }
  }

  pub fn get_name (&self) -> &str {
    match self.name.to_str() {
      Some(name) => name,
      None => "<unknown>",
    }
  }

  pub fn get_meta(&self) -> Option<&Meta> {
    match self.meta {
      Some(ref meta) => Some(meta),
      None => None,
    }
  }
}