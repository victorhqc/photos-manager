use crate::Thickness;
use console::{style, Emoji};
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use photos_manager_core::border::{add_border, Error as BorderError};
use snafu::prelude::*;
use std::{
    path::Path,
    sync::mpsc::{channel, sync_channel},
    thread,
    time::Instant,
};

static TRUCK: Emoji<'_, '_> = Emoji("🖼️  ", "");
static CAMERA: Emoji<'_, '_> = Emoji("📷 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");

pub fn border(source: String, from: Option<String>, thickness: Thickness) -> Result<()> {
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    let s = ProgressBar::new_spinner();
    s.set_style(spinner_style);

    let (t_tx, t_rx) = channel();
    let (p_tx, p_rx) = sync_channel(1);

    let thickness = match thickness {
        Thickness::Thin => 1,
        Thickness::Medium => 2,
        Thickness::Thick => 4,
    };

    thread::spawn(move || -> Result<()> {
        let source = Path::new(&source);
        add_border(
            source,
            from,
            thickness,
            |total| {
                if total == 1 {
                    s.finish_with_message(format!("   {}Adding border to your photo", CAMERA));
                } else {
                    s.finish_with_message(format!("   {}Found {} photos!", CAMERA, total));

                    println!("{} {}Adding borders...", style("[2/2]").bold().dim(), TRUCK);
                }
                t_tx.send(total).unwrap();
            },
            |current| {
                p_tx.send(Progress::Inc(current)).unwrap();
            },
            |_| {
                p_tx.send(Progress::Done).unwrap();
            },
        )
        .context(OrderSnafu)
    });

    let total = t_rx.recv().unwrap();
    let p = ProgressBar::new(total as u64);
    p.set_style(
        ProgressStyle::with_template(
            "{spinner:.green}     [{elapsed_precise}] [{wide_bar:.cyan/blue}]",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    for received in p_rx {
        match received {
            Progress::Inc(_) => {
                p.inc(1);
            }
            Progress::Done => {
                p.finish_and_clear();
                break;
            }
        }
    }

    if total > 1 {
        println!(
            "      {}Finish adding border to photos in {}!",
            CHECK,
            HumanDuration(started.elapsed())
        );
    } else {
        println!(
            "      {}Finish adding border in {}!",
            CHECK,
            HumanDuration(started.elapsed())
        );
    }

    Ok(())
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Add Border Error: {}", source))]
    Order { source: BorderError },
}

type Result<T, E = Error> = std::result::Result<T, E>;

enum Progress {
    Inc(u64),
    Done,
}
