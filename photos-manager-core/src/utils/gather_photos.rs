use crate::photo::Photo;
use log::{debug, warn};
use rayon::prelude::*;
use snafu::prelude::*;
use std::{
    io,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

pub fn gather_photos<'a>(dir: &Path) -> Result<Vec<Photo>> {
    debug!("Gathering photos from: {:?}", dir);

    let entries: Vec<Photo> = WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();
            println!("PATH {:?}", path);

            let extension = path
                .extension()
                .context(NoExtensionSnafu {
                    entry: entry.path(),
                })?
                .to_str()
                .unwrap();

            let file_type = entry.file_type();

            if !file_type.is_file() || !is_photo(extension) {
                return Ok(None);
            }

            debug!(
                "Extension: {:?} - Entry: {:?}",
                extension,
                entry.file_name()
            );

            Ok(Some(Photo {
                path: PathBuf::from(entry.path()),
                name: entry.file_name().to_os_string(),
            }))
        })
        // Ignore errors for now.
        .filter_map(|p: Result<Option<Photo>>| match p {
            Ok(p) => Some(p),
            Err(err) => {
                warn!("{:?}", err);
                None
            }
        })
        // Filter out none values.
        .filter_map(|p| p)
        .collect();

    Ok(entries)
}

fn is_photo(extension: &str) -> bool {
    match extension {
        "rgb" => true,
        "gif" => true,
        "pbm" => true,
        "pgm" => true,
        "ppm" => true,
        "tiff" => true,
        "rast" => true,
        "xbm" => true,
        "jpeg" => true,
        "jpg" => true,
        "bmp" => true,
        "png" => true,
        "webp" => true,
        "exr" => true,
        _ => false,
    }
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
