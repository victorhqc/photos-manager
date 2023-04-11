use crate::file::File;
use log::{debug, trace, warn};
use rayon::prelude::*;
use std::path::Path;
use walkdir::WalkDir;

pub fn gather_photos<F, D>(dir: &Path, gathering_fn: F, gather_done_fn: D) -> Vec<File>
where
    F: Fn(&File) + std::marker::Sync,
    D: FnOnce(usize),
{
    debug!("Gathering photos from: {:?}", dir);

    if dir.is_file() {
        let file = File::new(&dir.to_path_buf());
        let entries = match file {
            Ok(f) => vec![f],
            Err(err) => {
                warn!("Failed to add file: {:?}", err);
                vec![]
            }
        };

        return entries;
    }

    let entries: Vec<File> = WalkDir::new(dir)
        .into_iter()
        .par_bridge()
        .map(|e| {
            let entry = e.unwrap();
            let path = entry.path();

            if path.is_dir() {
                return Ok(None);
            }

            let file = match File::new(&path.to_path_buf()) {
                Ok(f) => f,
                Err(err) => {
                    warn!("Omitting: {:?}", err);

                    return Ok(None);
                }
            };

            trace!("Entry: {:?}", file.name());

            gathering_fn(&file);
            Ok(Some(file))
        })
        // Ignore errors for now.
        .filter_map(|p: Result<Option<File>, ()>| match p {
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
    entries
}
