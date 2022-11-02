use super::{get_created_at, GetCreatedAtError};
use crate::photo::Photo;
use log::trace;
use rayon::prelude::*;
use snafu::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn move_photos<F, D>(
    photos: &Vec<Photo>,
    target: &Path,
    ordering_fn: F,
    ordering_done_fn: D,
) -> Result<()>
where
    F: Fn(u64) + std::marker::Sync,
    D: FnOnce(usize),
{
    fs::create_dir_all(target).context(FailedToCreateTargetSnafu)?;

    let total = photos.len();
    photos
        .par_iter()
        .enumerate()
        .try_for_each(|(index, photo)| -> Result<()> {
            let created_at = get_created_at(photo).context(GetCreatedAtFailedSnafu)?;
            trace!("{:?}: {}", photo.name, created_at);

            let new_folder = format!("{}", created_at.format("%Y-%m"));
            let mut photo_target = PathBuf::from(target);
            photo_target.push(&new_folder);
            let targets = vec![photo.path.to_str().unwrap()];

            fs::create_dir_all(&photo_target)
                .context(FailedToCreatePhotoTargetSnafu { path: new_folder })?;

            let mut options = fs_extra::dir::CopyOptions::new();
            options.skip_exist = true;

            trace!("Moving {:?} to {:?}", photo.name, photo_target);
            fs_extra::move_items(&targets, &photo_target, &options)
                .context(CouldNotMovePhotoSnafu)?;

            ordering_fn(index as u64);

            Ok(())
        })?;

    ordering_done_fn(total);
    Ok(())
}

#[derive(Debug, Snafu)]
pub enum MovePhotosError {
    #[snafu(display("Unable to get created_at: {}", source))]
    GetCreatedAtFailed { source: GetCreatedAtError },

    #[snafu(display("Failed to create target path: {}", source))]
    FailedToCreateTarget { source: io::Error },

    #[snafu(display("Failed to create `{}` path: {}", path, source))]
    FailedToCreatePhotoTarget { source: io::Error, path: String },

    #[snafu(display("Photo was unable to move: {}", source))]
    CouldNotMovePhoto { source: fs_extra::error::Error },
}

pub type Result<T, E = MovePhotosError> = std::result::Result<T, E>;
