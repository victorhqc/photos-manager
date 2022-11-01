use std::{path::PathBuf, ffi::OsString};

#[derive(Clone, Debug)]
pub struct Photo {
    pub name: OsString,
    pub path: PathBuf,
}
