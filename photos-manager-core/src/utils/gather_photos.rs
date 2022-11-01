use crate::photo::Photo;
use log::{debug, warn};
use rayon::prelude::*;
use snafu::prelude::*;
use std::{fs, io, path::PathBuf};

pub fn gather_photos<'a>(pictures: &mut Vec<Photo>, dir: &PathBuf) -> Result<Vec<Photo>> {
    debug!("Gathering photos from: {:?}", dir);

    let entries: Vec<Photo> = fs::read_dir(dir)
        .context(ReadSourceSnafu)?
        .into_iter()
//        .into_par_iter()
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();

            if path.is_dir() {
                let nested: Vec<Photo> = gather_photos(pictures, &path)?;
                return Ok(Some(nested));
            }

            let extension = path
                .extension()
                .context(NoExtensionSnafu {
                    entry: entry.path(),
                })?
                .to_str()
                .unwrap();

            let file_type = entry.file_type().context(NoFileTypeSnafu {
                entry: entry.path(),
            })?;

            if !file_type.is_file() || !is_photo(extension) {
                return Ok(None);
            }

            debug!(
                "Extension: {:?} - Entry: {:?}",
                extension,
                entry.file_name()
            );

            Ok(Some(vec![Photo {
                path: entry.path(),
                name: entry.file_name(),
            }]))
        })
        // Ignore errors for now.
        .filter_map(|p: Result<Option<Vec<Photo>>>| match p {
            Ok(p) => Some(p),
            Err(err) => {
                warn!("{:?}", err);
                None
            }
        })
        // Filter out none values.
        .filter_map(|p| p)
        // Flatten vectors to handle nested paths.
        .flatten()
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
