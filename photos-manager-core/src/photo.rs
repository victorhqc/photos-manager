use std::{ffi::OsString, path::PathBuf};

#[derive(Clone, Debug)]
pub struct Photo {
    pub name: OsString,
    pub path: PathBuf,
}
