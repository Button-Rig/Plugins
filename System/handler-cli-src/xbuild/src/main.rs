use std::{fs, path::{self, Path}, process::Command};

struct Target<'a> {
    pub architecture: &'a str,
    pub release_file_name: &'a str,
    pub final_file_name: &'a str
}

fn main() {
    let targets: Vec<Target> = vec![
        Target {
            architecture: "x86_64-pc-windows-msvc",
            release_file_name: "handler-cli.exe",
            final_file_name: "handler-cli-x86-64-windows.exe"
        },
        Target {
            architecture: "i686-pc-windows-msvc",
            release_file_name: "handler-cli.exe",
            final_file_name: "handler-cli-x86-32-windows.exe"
        },
        // Target {
        //     architecture: "x86_64-apple-darwin",
        //     release_file_name: "handler-cli",
        //     final_file_name: "handler-cli-x86-64-mac"
        // },
        // Target {
        //     architecture: "aarch64-apple-darwin",
        //     release_file_name: "handler-cli",
        //     final_file_name: "handler-cli-arm-64-mac"
        // }
    ];

    for target in targets {
        let build_command = format!("cargo build --release --target={}", target.architecture);
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &build_command])
                .output()
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(&build_command)
                .output()
        };
        match output {
            Ok(output) => {
                println!("status : {}", output.status);
                println!("stdout : {}", &String::from_utf8_lossy(&output.stdout));
                eprintln!("stderr : {}", &String::from_utf8_lossy(&output.stderr));
                move_to_plugin_folder(target);
            },
            Err(e) => {
                eprintln!("error : {}", e);
            },
        }
    }
}


fn move_to_plugin_folder(target: Target) {
    let plugin_folder_path = Path::new("../plugin");
    let target_path_string = format!("./target/{}/release/{}", target.architecture, target.release_file_name);
    let target_path = Path::new(&target_path_string);
    if fs::exists(plugin_folder_path).unwrap() && fs::exists(target_path).unwrap() {
        let _ = fs::copy(target_path, plugin_folder_path.join(format!("{}", target.final_file_name)));
    }
}