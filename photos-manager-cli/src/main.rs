use clap::{Parser, Subcommand};

fn main() {
    let args = Arguments::parse();
    println!("{:?}", args);
    match args.cmd {
        SubCommand::Order { path } => {
            println!("This is the path: {:?}", path);
        }
    }
}
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
        path: String,
    },
}
