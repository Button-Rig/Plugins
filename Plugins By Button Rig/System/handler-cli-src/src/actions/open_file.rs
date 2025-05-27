use clap::Args;
use std::{
    fs,
    path::PathBuf,
    process::{Command, exit},
};

use super::process_result_handler::{handle_process_result, HandlerOptions};

#[derive(Debug, Args)]
pub struct OpenFile {
    #[arg(long)]
    file_path: PathBuf,
}

impl OpenFile {
    pub fn execute(&self) {        
        match fs::exists(&self.file_path) {
            Ok(exists) => match exists {
                true => println!("File exists!"),
                false => {
                    eprintln!("File does not exist.");
                    exit(1);
                }
            },
            Err(e) => {
                eprintln!("Error while checking if file exists : {}", e);
                exit(1);
            }
        }

        let result = match std::env::consts::OS {
            "linux" => Command::new("xdg-open").arg(&self.file_path).spawn(),
            "macos" => Command::new("open").arg(&self.file_path).spawn(),
            "windows" => Command::new("cmd")
                .args(["/C", "start", "", &self.file_path.to_string_lossy()])
                .spawn(),
            _ => {
                eprintln!("Unsupported operating system.");
                exit(1);
            }
        };
        handle_process_result(HandlerOptions {
            result: result,
            success_message: "File opened successfully.".to_string(),
            failure_message: "Something went wrong while openning the file.".to_string()
        });       
    }
}
