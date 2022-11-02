use crate::utils::{gather_photos, move_photos, GatherPhotosError, MovePhotosError};
use log::{debug, info};
use snafu::prelude::*;
use std::{io, path::Path};

pub fn order_photos(source: &Path, target: &Path) -> Result<()> {
    debug!("Ordering photos from path {:?}", source);
    debug!("Should place result in path {:?}", target);

    let photos = gather_photos(source).context(GatherFailedSnafu)?;
    info!("Found {} photos", photos.len());

    move_photos(&photos, target).context(MoveFailedSnafu)?;
    info!("Completed ordering {} photos!", photos.len());

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read source: {}", source))]
    ReadSource { source: io::Error },

    #[snafu(display("{:?}", source))]
    GatherFailed { source: GatherPhotosError },

    #[snafu(display("{:?}", source))]
    MoveFailed { source: MovePhotosError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
