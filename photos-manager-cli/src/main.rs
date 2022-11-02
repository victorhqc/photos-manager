use clap::{Parser, Subcommand};
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;
use photos_manager_core::order::{order_photos, Error as OrderError};
use snafu::prelude::*;
use std::path::Path;

fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Arguments::parse();
    debug!("Args: {:?}", args);

    match args.cmd {
        SubCommand::Order { source, target } => {
            let source = Path::new(&source);
            let target = Path::new(&target);

            order_photos(source, target).context(OrderIssueSnafu)?;
        }
    }

    Ok(())
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
