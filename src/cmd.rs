use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "mini-cache", version = "1.0", about = "A simple in-memory database")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands{
    Set { key: String, value: String },
    Get { key: String },
    Del { key: String },
    List,
}

