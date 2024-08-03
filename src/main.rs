extern crate base64;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine; // Import the Engine trait
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

const VERSION: &str = "1.0.0";

fn usage() {
    println!("Usage: password_manager [-a|--add name] [-r|--remove name] [-l|--list] [-f|--fetch name] [-v|--version]");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        std::process::exit(1);
    }

    let option = &args[1];
    let name = if args.len() > 2 { &args[2] } else { "" };

    let user_home = env::var("HOME").expect("HOME environment variable not set");
    let qpass_path = PathBuf::from(user_home).join(".QPASS");

    if !qpass_path.exists() {
        fs::create_dir_all(&qpass_path).expect("Failed to create directory");
    }

    match option.as_str() {
        "-a" | "--add" => {
            if name.is_empty() {
                usage();
                std::process::exit(1);
            }

            let mut password = String::new();
            print!("[ Enter password ]\n$ ");
            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin().read_line(&mut password).expect("Failed to read line");

            let encoded_password = base64_engine.encode(password.trim());
            let file_path = qpass_path.join(format!("{}.qpf", name));
            fs::write(file_path, encoded_password).expect("Failed to write password to file");
        }
        "-r" | "--remove" => {
            if name.is_empty() {
                usage();
                std::process::exit(1);
            }

            let file_path = qpass_path.join(format!("{}.qpf", name));
            if file_path.exists() {
                fs::remove_file(file_path).expect("Failed to remove file");
            } else {
                println!("No such file: {}", name);
            }
        }
        "-l" | "--list" => {
            let entries = fs::read_dir(qpass_path).expect("Failed to read directory");
            let mut files = Vec::new();

            for entry in entries {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                if path.extension().map(|e| e == "qpf").unwrap_or(false) {
                    if let Some(name) = path.file_stem() {
                        files.push(name.to_string_lossy().to_string());
                    }
                }
            }

            if files.is_empty() {
                println!("No .qpf password entries found.");
            } else {
                for file in files {
                    println!("{}", file);
                }
            }
        }
        "-f" | "--fetch" => {
            if name.is_empty() {
                usage();
                std::process::exit(1);
            }

            let file_path = qpass_path.join(format!("{}.qpf", name));
            if file_path.exists() {
                let encoded_password = fs::read_to_string(file_path).expect("Failed to read password from file");
                match base64_engine.decode(&encoded_password) {
                    Ok(decoded_password) => {
                        let password = String::from_utf8(decoded_password).expect("Failed to convert password to string");
                        println!("{}", password);
                    }
                    Err(err) => {
                        println!("Failed to decode password: {}", err);
                    }
                }
            } else {
                println!("No such file: {}", name);
            }
        }
        "-v" | "--version" => {
            println!("QPass v{}", VERSION);
        }
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}