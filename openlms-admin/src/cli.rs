use clap::{Parser, Subcommand};

use crate::commands::UserCmd;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    User(UserCmd),
}
