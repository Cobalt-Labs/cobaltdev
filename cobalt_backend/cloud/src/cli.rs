use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Server,
    Upload {
        file_path: String,
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
    List {
        #[arg(short, long, default_value = "ibrahim3595")]
        user: String,
    },
}