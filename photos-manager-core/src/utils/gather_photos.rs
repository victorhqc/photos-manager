use crate::photo::Photo;
use snafu::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn gather_photos<'a>(pictures: Vec<Photo>, dir: &Path) -> Result<Vec<Photo>> {
    let entries: Vec<Photo> = fs::read_dir(dir)
        .context(ReadSourceSnafu)?
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();

            if path.is_dir() {
                return Ok(None);
            }

            let extension = path.extension().context(NoExtensionSnafu { entry: entry.path() })?.to_str().unwrap();
            let file_type = entry.file_type().context(NoFileTypeSnafu {
                entry: entry.path(),
            })?;

            if !file_type.is_file() || !is_photo(extension) {
                return Ok(None);
            }

            println!("ENTRY: {:?}", entry.file_name());
            println!("EXTENSION {:?}", extension);

            Ok(Some(Photo {
                path: entry.path(),
                name: entry.file_name(),
            }))
        })
        // Ignore errors for now
        .filter_map(|p: Result<Option<Photo>>| p.ok())
        // Filter out none values
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
    NoExtension { entry: PathBuf }
}

pub type Result<T, E = GatherPhotosError> = std::result::Result<T, E>;
