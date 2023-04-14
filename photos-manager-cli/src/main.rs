use crate::cmds::{
    border::{border, Error as BorderError},
    order::{order, Error as OrderError},
};
use clap::{Parser, Subcommand, ValueEnum};
use dirs::home_dir;
use dotenv::dotenv;
use log::debug;
use snafu::prelude::*;
use strum_macros::Display;

mod cmds;

fn main() -> Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let args = Arguments::parse();
    debug!("Args: {:?}", args);

    match args.cmd {
        SubCommand::Order { source, target } => order(source, target).context(OrderSnafu),
        SubCommand::Border {
            source,
            from,
            thickness,
        } => border(source, from, thickness).context(BorderSnafu),
    }
}

#[derive(Debug, Snafu)]
enum CLIError {
    #[snafu(display("Ordering Error: {}", source))]
    Order { source: OrderError },

    #[snafu(display("Border Error: {}", source))]
    Border { source: BorderError },
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

    /// Add a white border to photos
    Border {
        /// Path to a photo: `C:\path\to\your\photos\my_pic.jpg`,`/path/to/your/photos/my_pic.jpg` or a directory to be applied to all pictures in it.
        #[clap(short, long, default_value_t = home_dir().unwrap().into_os_string().into_string().unwrap())]
        source: String,

        /// Date, in case of the source being a dir, the borders will be applied to pictures created after the provided date.
        #[clap(short, long)]
        from: Option<String>,

        /// Thickness of the border.
        #[clap(short, long, default_value_t = Thickness::Thin)]
        thickness: Thickness,
    },
}

#[derive(ValueEnum, Clone, Debug, Display)]
pub enum Thickness {
    #[strum(serialize = "thin")]
    Thin,
    #[strum(serialize = "medium")]
    Medium,
    #[strum(serialize = "thick")]
    Thick,
}
