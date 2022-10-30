use crate::utils::gather_photos;
use snafu::prelude::*;
use std::{io, path::Path};

pub fn order_photos(source: &Path, target: &Path) -> Result<()> {
    println!("Ordering photos from path {:?}", source);
    println!("Should place result in path {:?}", target);

    let pictures = gather_photos(vec![], source);

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to read source: {}", source))]
    ReadSource { source: io::Error },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
