use std::{thread, time::Duration};

use clap::Args;

#[derive(Debug, Args)]
pub struct Delay {
    #[arg(long)]
    milliseconds: u64
}

impl Delay {
    pub fn execute(&self) {
        thread::sleep(Duration::from_millis(self.milliseconds as u64));
        println!("Slept for {} milliseconds.", self.milliseconds);
    }
}