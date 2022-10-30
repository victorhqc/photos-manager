use std::{path::PathBuf, ffi::OsString};

pub struct Photo {
    pub name: OsString,
    pub path: PathBuf,
}
