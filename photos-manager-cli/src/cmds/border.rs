use photos_manager_core::border::{add_border_to, Error as BorderError};
use snafu::prelude::*;
use std::path::Path;

pub fn border(source: String) -> Result<()> {
    let source = Path::new(&source);
    add_border_to(source).context(OrderSnafu)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Add Border Error: {}", source))]
    Order { source: BorderError },
}

type Result<T, E = Error> = std::result::Result<T, E>;
