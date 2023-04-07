use snafu::prelude::*;

pub fn border(source: String) -> Result<()> {
    println!("Adding border to: {}", source);

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    Oops,
}

type Result<T, E = Error> = std::result::Result<T, E>;
