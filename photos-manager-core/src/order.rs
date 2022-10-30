use snafu::prelude::*;
use std::path::Path;

pub fn order_photos(in_path: &Path, out_path: &Path) -> Result<()> {
    println!("Ordering photos from path {:?}", in_path);
    println!("Should place result in path {:?}", out_path);

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    UnknownError
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
