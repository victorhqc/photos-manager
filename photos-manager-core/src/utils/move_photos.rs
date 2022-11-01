use super::{read_photo, GetCreatedAtError};
use crate::photo::Photo;
use log::debug;
use rayon::prelude::*;
use snafu::prelude::*;

pub fn move_photos(photos: &Vec<Photo>) -> Result<()> {
    photos.par_iter().try_for_each(|photo| -> Result<()> {
        let (_, created_at) = read_photo(photo).context(GetCreatedAtFailedSnafu)?;
        debug!("{:?}: {}", photo.name, created_at);

        Ok(())
    })?;

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum MovePhotosError {
    #[snafu(display("Unable to get created_at: {}", source))]
    GetCreatedAtFailed { source: GetCreatedAtError },
}

pub type Result<T, E = MovePhotosError> = std::result::Result<T, E>;
