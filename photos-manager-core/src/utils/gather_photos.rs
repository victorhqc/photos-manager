use crate::file::{File, Photo, Video};
use log::{debug, trace, warn};
use rayon::prelude::*;
use snafu::prelude::*;
use std::{
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn gather_photos<F, D>(dir: &Path, gathering_fn: F, gather_done_fn: D) -> Result<Vec<File>>
where
    F: Fn(&File) + std::marker::Sync,
    D: FnOnce(usize),
{
    debug!("Gathering photos from: {:?}", dir);

    let entries: Vec<File> = WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();

            if path.is_dir() {
                return Ok(None);
            }

            let extension = path
                .extension()
                .context(NoExtensionSnafu {
                    entry: entry.path(),
                })?
                .to_str()
                .unwrap();

            let file_type = entry.file_type();

            if !file_type.is_file() || !(is_photo(extension) || is_video(extension)) {
                return Ok(None);
            }

            trace!(
                "Extension: {:?} - Entry: {:?}",
                extension,
                entry.file_name()
            );

            let file = if is_video(extension) {
                File::Video(Video {
                    path: PathBuf::from(entry.path()),
                    name: entry.file_name().to_os_string(),
                })
            } else {
                File::Photo(Photo {
                    path: PathBuf::from(entry.path()),
                    name: entry.file_name().to_os_string(),
                })
            };

            gathering_fn(&file);
            Ok(Some(file))
        })
        // Ignore errors for now.
        .filter_map(|p: Result<Option<File>>| match p {
            Ok(p) => Some(p),
            Err(err) => {
                warn!("{:?}", err);
                None
            }
        })
        // Filter out none values.
        .filter_map(|p| p)
        .collect();

    gather_done_fn(entries.len());
    Ok(entries)
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

#[derive(Debug, Snafu)]
pub enum GatherPhotosError {
    #[snafu(display("Failed to read source: {}", source))]
    ReadSource { source: io::Error },

    #[snafu(display("Entry has no file type {}: {}", entry.display(), source))]
    NoFileType { source: io::Error, entry: PathBuf },

    #[snafu(display("Entry has no extension: {}", entry.display()))]
    NoExtension { entry: PathBuf },
}

pub type Result<T, E = GatherPhotosError> = std::result::Result<T, E>;
