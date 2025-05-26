use clap::Args;
use std::path::PathBuf;

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(long)]
    file_path: PathBuf,
    #[arg(long)]
    command: String
}

impl RunCommand {
    pub fn execute(&self) {
    }
}