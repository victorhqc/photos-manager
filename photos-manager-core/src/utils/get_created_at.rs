use crate::file::{File, Photo, Video};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime, ParseError};
use exif::{In, Tag};
use lazy_static::lazy_static;
use log::{trace, warn};
use regex::Regex;
use snafu::prelude::*;
use std::{fs::File as FsFile, io, time::UNIX_EPOCH};

pub fn get_created_at(file: &File) -> Result<NaiveDateTime> {
    match file {
        File::Photo(p) => get_created_from_photo(p),
        File::Video(v) => get_created_from_video(v),
    }
}

fn get_created_from_photo(photo: &Photo) -> Result<NaiveDateTime> {
    let path = &photo.path;
    let opened_file = FsFile::open(&photo.path).context(CouldNotOpenPhotoSnafu)?;

    trace!("Getting created at from photo: {:?}", photo.name);

    let mut bufreader = std::io::BufReader::new(&opened_file);
    let exifreader = exif::Reader::new();
    let exif = match exifreader
        .read_from_container(&mut bufreader)
        .context(PhotoHasNoExifDataSnafu)
    {
        Ok(e) => e,
        Err(_) => {
            warn!("{:?} has no exif data, using metadata instead.", path);

            return get_created_at_from_metadata(opened_file, photo.name.to_str().unwrap());
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

            return get_created_at_from_metadata(opened_file, photo.name.to_str().unwrap());
        }
    };
    let created_at = created_at.display_as(Tag::DateTime).to_string();
    let created_at = NaiveDateTime::parse_from_str(&created_at, "%Y-%m-%d %H:%M:%S")
        .context(FailedToParseDateSnafu)?;

    Ok(created_at)
}

fn get_created_from_video(video: &Video) -> Result<NaiveDateTime> {
    let opened_file = FsFile::open(&video.path).context(CouldNotOpenPhotoSnafu)?;

    // Ideally I'm able to use windows properties to get a video's date, but I'm unable to do it so far,
    // asked a question here:
    // https://learn.microsoft.com/en-us/answers/questions/1075226/how-to-use-folder-api-with-rust.html

    return get_created_at_from_metadata(opened_file, video.name.to_str().unwrap());
}

fn get_created_at_from_metadata(file: FsFile, filename: &str) -> Result<NaiveDateTime> {
    if let Ok(date) = get_created_at_from_name(filename) {
        return Ok(date);
    }

    let metadata = file.metadata().context(PhotoHasNoMetadataSnafu)?;
    let created_at = metadata.created().context(NoCreatedAtSnafu)?;
    let created_at_timestamp = created_at.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let created_at = NaiveDateTime::from_timestamp_opt(created_at_timestamp as i64, 0)
        .context(NameHasNoValidDateSnafu)?;

    Ok(created_at)
}

fn get_created_at_from_name(name: &str) -> Result<NaiveDateTime> {
    lazy_static! {
        // 1. YYYY-MM-DD
        // 2. _YYYYMMDD_
        // 3. -YYYYMMDD_
        // 4. BURSTYYYMMDD
        // 5. YYYYMMDD_000
        static ref RE: Regex = Regex::new(r"\d{4}-\d{2}-\d{2}|_\d{8}_|-\d{8}-|BURST\d{8}|(\d{8})_(\d{3,6})\.").unwrap();
    }

    if !RE.is_match(name) {
        return Err(GetCreatedAtError::NameHasNoValidDate);
    }

    let captures = RE.captures(name).unwrap();
    let date_match = match captures.get(1) {
        Some(dm) => dm.as_str(),
        None => captures.get(0).map_or("", |m| m.as_str()),
    };

    let date = date_match.replace(['-', '_', '.'], "").replace("BURST", "");
    let date = NaiveDate::parse_from_str(&date, "%Y%m%d").context(FailedToParseDateSnafu)?;
    let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();

    Ok(NaiveDateTime::new(date, time))
}

#[derive(Debug, Snafu)]
pub enum GetCreatedAtError {
    #[snafu(display("Failed to open file: {}", source))]
    CouldNotOpenPhoto { source: io::Error },

    #[snafu(display("Failed to access exif data: {}", source))]
    PhotoHasNoExifData { source: exif::Error },

    #[snafu(display("Photo has no metadata: {}", source))]
    PhotoHasNoMetadata { source: io::Error },

    #[snafu(display("Photo unable to reach created_at timestamp from file: {}", source))]
    NoCreatedAt { source: io::Error },

    #[snafu(display("Exif data from file has no field 'date_time_original'"))]
    NoDateTimeInExif,

    #[snafu(display("Failed to parse date: {}", source))]
    FailedToParseDate { source: ParseError },

    #[snafu(display("File has no valid date name"))]
    NameHasNoValidDate,

    #[snafu(display("Failed to access file metadata: {}", source))]
    CouldNotReadFileMetadata { source: io::Error },
}

pub type Result<T, E = GetCreatedAtError> = std::result::Result<T, E>;
