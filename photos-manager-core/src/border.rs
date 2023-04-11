use log::debug;
use magick_rust::{bindings, magick_wand_genesis, MagickError, MagickWand, PixelWand, HSL};
use snafu::prelude::*;
use std::path::Path;
use std::sync::Once;

static START: Once = Once::new();
static WHITE: HSL = HSL {
    hue: 0.0,
    saturation: 0.0,
    lightness: 100.0,
};

pub fn add_border_to(photo: &Path) -> Result<()> {
    debug!("Adding white border to {:?}", photo);

    let path = photo.to_str().unwrap();

    START.call_once(|| {
        magick_wand_genesis();
    });

    let wand = MagickWand::new();
    let pixel = PixelWand::new();
    let operator = bindings::CompositeOperator_SrcOverCompositeOp;

    pixel.set_hsl(&WHITE);

    wand.read_image(path).context(ReadSnafu)?;
    let border = get_border_width(&wand)?;

    wand.border_image(&pixel, border, border, operator)
        .context(BorderSnafu)?;

    wand.write_image(path).context(WriteSnafu)?;

    Ok(())
}

fn get_border_width(wand: &MagickWand) -> Result<usize> {
    let width = wand.get_image_width();
    let height = wand.get_image_height();

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
        Format::Square => width * 0.01,
        Format::Portrait => width * 0.01,
        Format::Landscape => height * 0.01,
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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
