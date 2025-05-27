use clap::Args;
use std::{path::PathBuf, process::exit};

#[derive(Debug, Args)]
pub struct OpenFile {
    #[arg(long)]
    file_path: PathBuf,
}

impl OpenFile {
    pub fn execute(&self) {
        let result = open::that(self.file_path.clone());

        if let Err(e) = result {
            eprintln!("Error while trying to open file : {}", e);
            exit(1);
        }
    }
}
