use super::{get_created_at, GetCreatedAtError};
use crate::photo::Photo;
use log::debug;
use rayon::prelude::*;
use snafu::prelude::*;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

pub fn move_photos(photos: &Vec<Photo>, target: &Path) -> Result<()> {
    fs::create_dir_all(target).context(FailedToCreateTargetSnafu)?;

    photos.par_iter().try_for_each(|photo| -> Result<()> {
        let created_at = get_created_at(photo).context(GetCreatedAtFailedSnafu)?;
        debug!("{:?}: {}", photo.name, created_at);

        let new_folder = format!("{}", created_at.format("%Y-%m"));
        let mut photo_target = PathBuf::from(target);
        photo_target.push(&new_folder);
        let targets = vec![photo.path.to_str().unwrap()];

        fs::create_dir_all(&photo_target)
            .context(FailedToCreatePhotoTargetSnafu { path: new_folder })?;

        let mut options = fs_extra::dir::CopyOptions::new();
        options.skip_exist = true;

        debug!("Moving {:?} to {:?}", photo.name, photo_target);
        fs_extra::move_items(&targets, &photo_target, &options).context(CouldNotMovePhotoSnafu)?;

        Ok(())
    })?;

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
