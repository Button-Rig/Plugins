use clap::Args;
use std::{path::PathBuf, process::exit};

#[derive(Debug, Args)]
pub struct OpenFolder {
    #[arg(long)]
    folder_path: PathBuf,
}

impl OpenFolder {
    pub fn execute(&self) {
        let result = open::that(self.folder_path.clone());
        
        if let Err(e) = result {
            eprintln!("Error while trying to open the folder : {}", e);
            exit(1);
        }
    }
}
