use std::{io::Error, process::{exit, Child}};

pub struct HandlerOptions {
    pub result: Result<Child, Error>,
    pub success_message: String,
    pub failure_message: String
}

pub fn handle_process_result(options: HandlerOptions) {
    match options.result {
        Ok(child) => match child.wait_with_output() {
            Ok(output) => {
                let log = &String::from_utf8_lossy(&output.stdout);
                println!("{}", log);
                let error_log = &String::from_utf8_lossy(&output.stderr);
                eprintln!("{}", error_log);
                match output.status.code() {
                    Some(exit_code) => {
                        if exit_code == 0 {
                            println!("{}", options.success_message);
                            return;
                        } else {
                            eprintln!(
                                "{} Process exited with status code: {}",
                                options.failure_message,
                                exit_code
                            );
                            exit(1);
                        }
                    }
                    None => {
                        eprintln!(
                            "{} Process exited with no status code.",
                            options.failure_message
                        );
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("{} : {}", options.failure_message, e);
                exit(1);
            }
        },
        Err(e) => {
            eprintln!("{} : {}", options.failure_message, e);
            exit(1);
        }
    }
}
