use clap::Args;
use std::{
    io::Error,
    process::{Child, Command, exit},
};

#[derive(Debug, Args)]
pub struct RunCommand {
    #[arg(long)]
    path: Option<String>,
    #[arg(long)]
    command: String,
}

impl RunCommand {
    pub fn execute(&self) {
        let result = if cfg!(target_os = "windows") {
            let mut command = Command::new("cmd");
            command.args(["/C", &self.command]);
            if let Some(path) = &self.path {
                command.current_dir(path);
            }
            command.spawn()
        } else {
            let mut command = Command::new("sh");

            command.arg("-c").arg(&self.command);

            if let Some(path) = &self.path {
                command.current_dir(&path);
            }
            command.spawn()
        };

        handle_process_result(
            result,
            "Command run successfully.",
            "Something went wrong while running the command.",
        );
    }
}

pub fn handle_process_result(
    result: Result<Child, Error>,
    success_message: &str,
    failure_message: &str,
) {
    match result {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => {
                let log = &String::from_utf8_lossy(&output.stdout);
                println!("{}", log);
                let error_log = &String::from_utf8_lossy(&output.stderr);
                eprintln!("{}", error_log);
                match output.status.code() {
                    Some(exit_code) => {
                        if exit_code == 0 {
                            println!("{}", success_message);
                            return;
                        } else {
                            eprintln!(
                                "{} Process exited with status code: {}",
                                failure_message, exit_code
                            );
                            exit(1);
                        }
                    }
                    None => {
                        eprintln!("{} Process exited with no status code.", failure_message);
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("{} : {}", failure_message, e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("{} : {}", failure_message, e);
            exit(1);
        }
    }
}
