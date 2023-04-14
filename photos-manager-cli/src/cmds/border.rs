use crate::Thickness;
use photos_manager_core::border::{add_border_to, Error as BorderError};
use snafu::prelude::*;
use std::path::Path;

pub fn border(source: String, from: Option<String>, thickness: Thickness) -> Result<()> {
    let source = Path::new(&source);

    let thickness = match thickness {
        Thickness::Thin => 1,
        Thickness::Medium => 2,
        Thickness::Thick => 4,
    };

    add_border_to(source, from, thickness).context(OrderSnafu)
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Add Border Error: {}", source))]
    Order { source: BorderError },
}

type Result<T, E = Error> = std::result::Result<T, E>;
