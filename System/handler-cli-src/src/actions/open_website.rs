use std::process::exit;
use clap::Args;

#[derive(Debug, Args)]
pub struct OpenWebsite {
    #[arg(long)]
    url: String,
}

impl OpenWebsite {
    pub fn execute(&self) {
        let url = if self.url.contains("http://") || self.url.contains("https://") {
            &self.url
        } else {
            &format!("https://{}", self.url)
        };

        let result = open::that(url);

        if let Err(e) = result {
            eprintln!("Error while opening the website : {}", e);
            exit(1);
        }
    }
}
