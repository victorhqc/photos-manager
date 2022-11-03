use crate::{
    file::File,
    utils::{gather_photos, move_photos, GatherPhotosError, MovePhotosError},
};
use log::{debug, info};
use snafu::prelude::*;
use std::{io, path::Path};

pub fn order_photos<F, G, H, I>(
    source: &Path,
    target: &Path,
    gathering_fn: F,
    gathering_done_fn: G,
    moving_fn: H,
    moving_done_fn: I,
) -> Result<()>
where
    F: Fn(&File) + std::marker::Sync,
    G: FnOnce(usize),
    H: Fn(u64) + std::marker::Sync,
    I: FnOnce(usize),
{
    debug!("Ordering photos from path {:?}", source);
    debug!("Should place result in path {:?}", target);

    let photos =
        gather_photos(source, gathering_fn, gathering_done_fn).context(GatherFailedSnafu)?;
    info!("Found {} photos", photos.len());

    move_photos(&photos, target, moving_fn, moving_done_fn).context(MoveFailedSnafu)?;
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
