use snafu::prelude::*;
use std::{ffi::OsString, fs, io, path::PathBuf};

pub enum File {
    Photo(Photo),
    Video(Video),
}

impl File {
    pub fn new(path: &PathBuf) -> Result<Self> {
        let metadata = fs::metadata(path).context(MetadataSnafu)?;

        let extension = path
            .extension()
            .context(NoExtensionSnafu { entry: path })?
            .to_str()
            .unwrap();

        let file_type = metadata.file_type();

        if !file_type.is_file() || !(is_photo(extension) || is_video(extension)) {
            return Err(Error::InvalidFile);
        }

        let name = path
            .file_name()
            .context(MissingFileNameSnafu)?
            .to_os_string();

        let file = if is_video(extension) {
            File::Video(Video {
                path: path.clone(),
                name,
            })
        } else {
            File::Photo(Photo {
                path: path.clone(),
                name,
            })
        };

        Ok(file)
    }

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

fn is_photo(extension: &str) -> bool {
    matches!(
        extension.to_lowercase().as_str(),
        "rgb"
            | "gif"
            | "pbm"
            | "pgm"
            | "ppm"
            | "tiff"
            | "rast"
            | "xbm"
            | "jpeg"
            | "jpg"
            | "bmp"
            | "png"
            | "webp"
            | "exr"
            | "heif"
    )
}

fn is_video(extension: &str) -> bool {
    matches!(
        extension.to_lowercase().as_str(),
        "mp4" //            | "m4p"
              //            | "webm"
              //            | "mpg"
              //            | "mp2"
              //            | "mpeg"
              //            | "mpe"
              //            | "mpv"
              //            | "ogg"
              //            | "m4v"
              //            | "avi"
              //            | "wmv"
              //            | "mov"
              //            | "flv"
              //            | "swf"
              //            | "acchd"
              //            | "qt"
    )
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

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read metadata: {}", source))]
    Metadata { source: io::Error },

    #[snafu(display("File is not valid"))]
    InvalidFile,

    #[snafu(display("Missing file name"))]
    MissingFileName,

    #[snafu(display("Failed to read source: {}", source))]
    ReadSource { source: io::Error },

    #[snafu(display("Entry has no file type {}: {}", entry.display(), source))]
    NoFileType { source: io::Error, entry: PathBuf },

    #[snafu(display("Entry has no extension: {}", entry.display()))]
    NoExtension { entry: PathBuf },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
