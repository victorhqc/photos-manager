use crate::photo::Photo;
use chrono::{NaiveDateTime, ParseError};
use exif::{In, Tag};
use log::warn;
use snafu::prelude::*;
use std::{fs::File, io, time::UNIX_EPOCH};

pub fn get_created_at(photo: &Photo) -> Result<NaiveDateTime> {
    let path = &photo.path;
    let opened_photo = File::open(&photo.path).context(CouldNotOpenPhotoSnafu)?;

    let mut bufreader = std::io::BufReader::new(&opened_photo);
    let exifreader = exif::Reader::new();
    let exif = match exifreader
        .read_from_container(&mut bufreader)
        .context(PhotoHasNoExifDataSnafu)
    {
        Ok(e) => e,
        Err(_) => {
            warn!("{:?} has no exif data, using metadata instead.", path);

            return get_created_at_from_metadata(opened_photo);
        }
    };

    let created_at = match exif
        .get_field(Tag::DateTimeOriginal, In::PRIMARY)
        .context(NoDateTimeInExifSnafu)
    {
        Ok(exif) => &exif.value,
        Err(_) => {
            warn!(
                "{:?} exif data has no date_time, using metadata instead",
                path
            );

            return get_created_at_from_metadata(opened_photo);
        }
    };
    let created_at = created_at.display_as(Tag::DateTime).to_string();
    let created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")
        .context(FailedToParseDateSnafu)?;

    Ok(created_at)
}

fn get_created_at_from_metadata(file: File) -> Result<NaiveDateTime> {
    let metadata = file.metadata().context(PhotoHasNoMetadataSnafu)?;
    let created_at = metadata.created().context(NoCreatedAtSnafu)?;
    let created_at_timestamp = created_at.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let created_at = NaiveDateTime::from_timestamp(created_at_timestamp as i64, 0);

    Ok(created_at)
}

#[derive(Debug, Snafu)]
pub enum GetCreatedAtError {
    #[snafu(display("Failed to open photo: {}", source))]
    CouldNotOpenPhoto { source: io::Error },

    #[snafu(display("Failed to access exif data: {}", source))]
    PhotoHasNoExifData { source: exif::Error },

    #[snafu(display("Photo has no metadata: {}", source))]
    PhotoHasNoMetadata { source: io::Error },

    #[snafu(display("Photo unable to reach created_at timestamp from photo: {}", source))]
    NoCreatedAt { source: io::Error },

    #[snafu(display("Exif data from photo has no field 'date_time_original'"))]
    NoDateTimeInExif,

    #[snafu(display("Failed to parse date: {}", source))]
    FailedToParseDate { source: ParseError },
}

pub type Result<T, E = GetCreatedAtError> = std::result::Result<T, E>;
