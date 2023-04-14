use crate::{
    file::File,
    utils::{gather_photos, get_created_at, GetCreatedAtError},
};
use chrono::{Datelike, NaiveDate};
use log::{debug, warn};
use magick_rust::{bindings, magick_wand_genesis, MagickError, MagickWand, PixelWand, HSL};
use snafu::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::sync::Once;

static START: Once = Once::new();
static WHITE: HSL = HSL {
    hue: 0.0,
    saturation: 0.0,
    lightness: 100.0,
};

pub fn add_border_to(path: &Path, from: Option<String>, thickness: u8) -> Result<()> {
    debug!("Adding white border to {:?}", path);

    let photos = gather_photos(path, |_| {}, |_| {});
    let from: Option<NaiveDate> = match from {
        Some(f) => {
            if path.is_dir() {
                let date = NaiveDate::from_str(&f).context(BadDateSnafu)?;
                Some(date)
            } else {
                None
            }
        }
        None => None,
    };

    START.call_once(|| {
        magick_wand_genesis();
    });

    let operator = bindings::CompositeOperator_SrcOverCompositeOp;
    let pixel = PixelWand::new();
    pixel.set_hsl(&WHITE);

    for photo in photos {
        // Skip Videos
        match photo {
            File::Video(_) => {
                continue;
            }
            File::Photo(_) => {}
        };

        // Skip photos that are older than the provided date.
        match from {
            None => {}
            Some(f) => {
                let created_at = get_created_at(&photo).context(MissingMetadataSnafu)?;

                let created_at: NaiveDate = NaiveDate::from_ymd_opt(
                    created_at.year(),
                    created_at.month(),
                    created_at.day(),
                )
                .unwrap();

                let before = created_at < f;
                debug!("{:?} < {:?} = {}", created_at, f, before);

                if before {
                    warn!("Skipping photo: {:?}", photo.name());
                    continue;
                }
            }
        }

        let wand = MagickWand::new();

        let path = photo.path().to_str().unwrap();
        wand.read_image(path).context(ReadSnafu)?;

        let border = get_border_width(&wand, thickness)?;
        wand.border_image(&pixel, border, border, operator)
            .context(BorderSnafu)?;

        wand.write_image(path).context(WriteSnafu)?;
    }

    Ok(())
}

fn get_border_width(wand: &MagickWand, thickness: u8) -> Result<usize> {
    let width = wand.get_image_width();
    let height = wand.get_image_height();

    let thickness: f32 = thickness as f32 / 100.0;

    let format: Format = if width == height {
        Format::Square
    } else if width < height {
        Format::Portrait
    } else {
        Format::Landscape
    };

    debug!("Image format: {:?}", format);

    let width = width as f32;
    let height = height as f32;

    let border: f32 = match format {
        Format::Square => width * thickness,
        Format::Portrait => width * thickness,
        Format::Landscape => height * thickness,
    };

    let border: i32 = border.round() as i32;
    let border = usize::try_from(border).unwrap();

    // Prevent border from being less than 20 pixels
    let border = if border < 20 { 20 } else { border };
    debug!("Border width: {:?}", border);

    Ok(border)
}

#[derive(Debug)]
pub enum Format {
    Square,
    Portrait,
    Landscape,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Failed to open image: {:?}", source))]
    Read { source: MagickError },

    #[snafu(display("Failed to apply border: {:?}", source))]
    Border { source: MagickError },

    #[snafu(display("Failed to write image: {:?}", source))]
    Write { source: MagickError },

    #[snafu(display("Failed to parse date: {:?}", source))]
    BadDate { source: chrono::ParseError },

    #[snafu(display("Unable to figure out the creation date: {:?}", source))]
    MissingMetadata { source: GetCreatedAtError },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
