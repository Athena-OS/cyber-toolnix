mod utils;
use crate::utils::*;
use std::{env, fs, str};
use std::io::stdin;
use std::process::Command;

fn main() {

    let args: Vec<String> = env::args().collect();
    let config_file = format!("/etc/nixos/configuration.nix");

    if !file_exists(&config_file) {
        println!("File does not exist: {}", config_file);
        return;
    }

    if args.len() < 2 {
        // Handle the case where no command-line arguments are provided
        match print_banner() {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error: {}", error);
            }
        }
        get_help();
        return;
    }
    
    let _ = print_banner();

    match args[1].as_str() {
        "blue" | "bounty" | "cracker" | "dos" | "forensic" | "malware" | "mobile" | "network" | "osint" | "red" | "student" | "web" => {
            println!("\nSetting {} role...\n", args[1].as_str());
            set_role(args[1].as_str(), &config_file);
        }
        _ => {
            println!("Invalid command: {}", args[1]);
            get_help();
        }
    }

    let mut current_user = String::new();
    let output = Command::new("who") // It is the only command to get the username calling sudo cyber-toolkit
        .output()
        .expect("Failed to execute 'who' command");

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout).expect("Failed to parse UTF-8");
        let username = stdout.split_whitespace().next().unwrap_or("");
        current_user = username.to_string();
    } else {
        eprintln!("Error: 'who' command failed");
    }
    let setting_file = format!("/home/{}/.config/athena-welcome/settings.conf", current_user);

    if fs::metadata(setting_file.clone()).is_ok() {
        exec_eval(
            exec(
                "sed",
                vec![
                    String::from("-in"),
                    format!("s/^role=.*/role={}/g", args[1].as_str()),
                    setting_file,
                ],
            ),
            "Delete commented lines from file",
        );
    }
    println!("All done. Your role has been set!");
    
    let mut input = String::new();
    println!("Press Enter to continue");
    stdin().read_line(&mut input).expect("Failed to read input");
}

fn file_exists(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}