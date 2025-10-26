use std::{io::Error, process::{exit, Child, Command}};

pub fn run_command(command: &str) {
    let result = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", command]).spawn()
    } else {
        Command::new("sh").arg("-c").arg(command).spawn()
    };

    handle_process_result(
        result,
        "Command run successfully.",
        "Something went wrong while running the command.",
    );
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
