use clap::Args;
use std::{path::PathBuf, process::Command};

use super::process_result_handler::{handle_process_result, HandlerOptions};

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(long)]
    path: PathBuf,
    #[arg(long)]
    command: String,
}

impl RunCommand {
    pub fn execute(&self) {
        let result = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &self.command])
                .current_dir(&self.path)
                .spawn()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&self.command)
                .current_dir(&self.path)
                .spawn()
        };

        handle_process_result(HandlerOptions {
            result: result,
            success_message: "Command run successfully.".to_string(),
            failure_message: "Something went wrong while running the command.".to_string()
        });  
    }
}
