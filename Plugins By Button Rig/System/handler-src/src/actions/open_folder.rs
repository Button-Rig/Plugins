use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct OpenFolder {
    #[arg(long)]
    folder_path: PathBuf
}

impl OpenFolder {
    pub fn execute(&self) {
    }
}