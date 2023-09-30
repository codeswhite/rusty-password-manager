use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Store path
    #[arg(value_name = "Store File")]
    pub store_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Open a store (list entries or get entry)
    Open { entry_name: Option<String> },
    /// Create new store
    Create { store_name: Option<String> },
    /// Add an entry to an existing store
    Add { entry_name: Option<String> },
    /// Remove an entry from an existing store
    Remove { entry_name: Option<String> },
}
