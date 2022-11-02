use clap::{Parser, Subcommand};
use console::{style, Emoji};
use dirs::home_dir;
use dotenv::dotenv;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use log::debug;
use photos_manager_core::order::{order_photos, Error as OrderError};
use snafu::prelude::*;
use std::{
    path::Path,
    sync::mpsc::{channel, sync_channel},
    thread,
    time::Instant,
};

static LOOKING_GLASS: Emoji<'_, '_> = Emoji("🔍  ", "");
static TRUCK: Emoji<'_, '_> = Emoji("🚚  ", "");
static CAMERA: Emoji<'_, '_> = Emoji("📷 ", "");
static CHECK: Emoji<'_, '_> = Emoji("✅ ", "");

fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Arguments::parse();
    debug!("Args: {:?}", args);

    match args.cmd {
        SubCommand::Order { source, target } => order(source, target),
    }
}

fn order(source: String, target: String) -> Result<()> {
    let started = Instant::now();
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    println!(
        "{} {}Gathering photos...",
        style("[1/2]").bold().dim(),
        LOOKING_GLASS
    );

    let s = ProgressBar::new_spinner();
    s.set_style(spinner_style.clone());

    let (t_tx, t_rx) = channel();
    let (p_tx, p_rx) = sync_channel(1);

    thread::spawn(move || -> Result<()> {
        let source = Path::new(&source);
        let target = Path::new(&target);

        order_photos(
            source,
            target,
            |p| {
                s.set_message(format!("{:?}", p.name));
            },
            |total| {
                s.finish_with_message(format!("   {}Found {} photos!", CAMERA, total));

                println!("{} {}Moving photos...", style("[2/2]").bold().dim(), TRUCK);
                t_tx.send(total).unwrap();
            },
            |current| {
                p_tx.send(Progress::Inc(current)).unwrap();
            },
            |_| {
                p_tx.send(Progress::Done).unwrap();
            },
        )
        .context(OrderIssueSnafu)?;

        Ok(())
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

    println!(
        "      {}Finish ordering photos in {}!",
        CHECK,
        HumanDuration(started.elapsed())
    );

    Ok(())
}

enum Progress {
    Inc(u64),
    Done,
}

#[derive(Debug, Snafu)]
enum CLIError {
    #[snafu(display("Ordering Error: {}", source))]
    OrderIssue { source: OrderError },
}

type Result<T, E = CLIError> = std::result::Result<T, E>;

#[derive(Parser, Debug)]
#[clap(
    author = "Victor Quiroz Castro",
    version,
    about = "Utilities for photos"
)]
struct Arguments {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    /// Order photos by date (YYYY-mm)
    Order {
        /// Path to your photos: `C:\path\to\your\photos` or `/path/to/your/photos` depending on your OS
        #[clap(short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap())]
        source: String,

        /// Path where you want to place your ordered photos
        #[clap(short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap())]
        target: String,
    },
}
