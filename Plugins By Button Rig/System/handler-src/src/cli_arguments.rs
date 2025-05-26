use clap::{Parser, Subcommand};
use crate::actions::*;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    CopyToClipboard(CopyToClipboard),
    Delay(Delay),
    OpenFile(OpenFile),
    OpenFolder(OpenFolder),
    OpenWebsite(OpenWebsite),
    RunCommand(RunCommand),
}
