use clap::Args;

#[derive(Debug, Args)]
pub struct OpenWebsite {
    #[arg(long)]
    url: String
}

impl OpenWebsite {
    pub fn execute(&self) {
    }
}