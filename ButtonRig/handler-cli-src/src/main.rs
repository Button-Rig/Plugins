use clap::{Parser, Subcommand};

mod command_runner;
use command_runner::run_command;

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Close => run_command("buttonrig close"),
        Command::Exit => run_command("buttonrig exit"),
        Command::Lock => run_command("buttonrig lock"),
        Command::Open => run_command("buttonrig open"),
        Command::Unlock => run_command("buttonrig unlock"),
    }
}



#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Close,
    Exit,
    Lock,
    Open,
    Unlock,
}
