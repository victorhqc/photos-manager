use std::{ffi::OsString, path::PathBuf};

pub enum File {
    Photo(Photo),
    Video(Video),
}

impl File {
    pub fn path(&self) -> &PathBuf {
        match self {
            File::Photo(p) => &p.path,
            File::Video(v) => &v.path,
        }
    }

    pub fn name(&self) -> &OsString {
        match self {
            File::Photo(p) => &p.name,
            File::Video(v) => &v.name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Photo {
    pub name: OsString,
    pub path: PathBuf,
}

#[derive(Clone, Debug)]
pub struct Video {
    pub name: OsString,
    pub path: PathBuf,
}
