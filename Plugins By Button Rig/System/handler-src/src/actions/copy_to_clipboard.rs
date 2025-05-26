use std::process::exit;

use clap::Args;
use copypasta::{ClipboardContext, ClipboardProvider};

#[derive(Debug, Args)]
pub struct CopyToClipboard {
    #[arg(long)]
    value: String
}

impl CopyToClipboard {
    pub fn execute(&self) {
        let mut ctx = ClipboardContext::new().unwrap();
        let result = ctx.set_contents(self.value.clone());
        match result {
            Ok(_) => {
                println!("'{}' copied to clipboard sucessfully.", self.value);
            },
            Err(e) => {
                eprintln!("Error while copying to clipboard: {}", e);
                exit(1);
            },
        }
    }
}