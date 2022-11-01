use crate::utils::{gather_photos, GatherPhotosError};
use log::{debug, info};
use snafu::prelude::*;
use std::{
    io,
    path::{Path, PathBuf},
};

pub fn order_photos(source: &Path, target: &Path) -> Result<()> {
    debug!("Ordering photos from path {:?}", source);
    debug!("Should place result in path {:?}", target);

    let photos = gather_photos(&PathBuf::from(source)).context(GatherFailedSnafu)?;

    info!("Found {} photos", photos.len());
    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read source: {}", source))]
    ReadSource { source: io::Error },

    #[snafu(display("{:?}", source))]
    GatherFailed { source: GatherPhotosError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
