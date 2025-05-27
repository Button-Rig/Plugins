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
        let mut ctx = get_clipbaord_context();
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

fn get_clipbaord_context() -> ClipboardContext {
     return match ClipboardContext::new() {
            Ok(ctx) => ctx,
            Err(e) => {
                eprintln!("Error while fetching clipbaord context : {}", e);
                exit(1);
            },
        };
}