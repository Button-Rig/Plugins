use std::process::{exit, Command};

use clap::Args;

use super::process_result_handler::{handle_process_result, HandlerOptions};

#[derive(Debug, Args)]
pub struct OpenWebsite {
    #[arg(long)]
    url: String,
}

impl OpenWebsite {
    pub fn execute(&self) {
        let url = if self.url.contains("http://") || self.url.contains("https://") {
            self.url.clone()
        } else {
            format!("https://{}", self.url)
        };

        let result = match std::env::consts::OS {
            "linux" => Command::new("xdg-open").arg(url).spawn(),
            "macos" => Command::new("open").arg(url).spawn(),
            "windows" => Command::new("cmd").args(["/C", "start", &url]).spawn(),
            _ => {
                eprintln!("Unsupported operating system.");
                exit(1);
            }
        };

        handle_process_result(HandlerOptions {
            result: result,
            success_message: "Website opened successfully.".to_string(),
            failure_message: "Something went wrong while openning the website.".to_string()
        });  
    }
}
