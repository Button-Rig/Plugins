use clap::Args;
use std::{path::PathBuf, process::{exit, Command}};

use super::process_result_handler::{handle_process_result, HandlerOptions};

#[derive(Debug, Args)]
pub struct OpenFolder {
    #[arg(long)]
    folder_path: PathBuf,
}

impl OpenFolder {
    pub fn execute(&self) {
        let result = match std::env::consts::OS {
            "linux" => Command::new("xdg-open").arg(&self.folder_path).spawn(),
            "macos" => Command::new("open").arg(&self.folder_path).spawn(),
            "windows" => Command::new("cmd")
                .args(["/C", "start", "", &self.folder_path.to_string_lossy()])
                .spawn(),
            _ => {
                eprintln!("Unsupported operating system.");
                exit(1);
            }
        };

        handle_process_result(HandlerOptions {
            result: result,
            success_message: "Folder opened successfully.".to_string(),
            failure_message: "Something went wrong while openning the folder.".to_string()
        });  
    }
}
