mod cli_arguments;
mod actions;

use clap::Parser;
use cli_arguments::*;

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::CopyToClipboard(copy_to_clipboard) => copy_to_clipboard.execute(),
        Command::Delay(delay) => delay.execute(),
        Command::OpenFile(open_file) => open_file.execute(),
        Command::OpenFolder(open_folder) => open_folder.execute(),
        Command::OpenWebsite(open_website) => open_website.execute(),
        Command::RunCommand(run_command) => run_command.execute(),
    }
}
