extern crate sodiumoxide;
use sodiumoxide::crypto::secretbox;
use sodiumoxide::crypto::secretbox::{seal, open};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use colored::*; 

const VERSION: &str = "1.0.0";

// CHNAGE THIS TO YOUR OWN KEY
const HARDCODED_KEY: [u8; 32] = [0x1f, 0xa6, 0x8b, 0x5b, 0x4a, 0xe2, 0x0e, 0x73, 0x3e, 0x77, 0xc1, 0xa3, 0xf4, 0xa9, 0x7d, 0x4b, 0xf7, 0x2a, 0xe0, 0x4e, 0xc7, 0xe2, 0x7a, 0x9d, 0x78, 0x0c, 0xc3, 0x62, 0xe1, 0x1d, 0x26, 0x12];

fn usage() {
    println!("{}", "Usage: password_manager [-a|--add name] [-r|--remove name] [-l|--list] [-f|--fetch name] [-v|--version]".red());
}

fn main() {
    sodiumoxide::init().expect("Failed to initialize sodiumoxide");

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

    // Use the hardcoded key for encryption/decryption
    let key = secretbox::Key::from_slice(&HARDCODED_KEY).expect("Invalid key length");

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

            let nonce = secretbox::gen_nonce();
            let encrypted_password = seal(password.trim().as_bytes(), &nonce, &key);
            let file_path = qpass_path.join(format!("{}.txt", name));
            fs::write(file_path, [&nonce.0[..], &encrypted_password[..]].concat()).expect("Failed to write password to file");
            println!("{}", "Password added successfully!".green());
        }
        "-r" | "--remove" => {
            if name.is_empty() {
                usage();
                std::process::exit(1);
            }

            let file_path = qpass_path.join(format!("{}.txt", name));
            if file_path.exists() {
                fs::remove_file(file_path).expect(&"Failed to remove entry".red().to_string());
            } else {
                println!("No such file: {}", name.red());
            }
        }
        "-l" | "--list" => {
            let entries = fs::read_dir(qpass_path).expect(&"Failed to read directory".red().to_string());
            let mut files = Vec::new();

            for entry in entries {
                let entry = entry.expect(&"Failed to read entry".red().to_string());
                let path = entry.path();
                if path.extension().map(|e| e == "txt").unwrap_or(false) {
                    if let Some(name) = path.file_stem() {
                        files.push(name.to_string_lossy().to_string());
                    }
                }
            }

            if files.is_empty() {
                println!("{}", "No qpass password entries found.".red());
            } else {
                for file in files {
                    println!("{}", file.green());
                }
            }
        }
        "-f" | "--fetch" => {
            if name.is_empty() {
                usage();
                std::process::exit(1);
            }

            let file_path = qpass_path.join(format!("{}.txt", name));
            if file_path.exists() {
                let data = fs::read(file_path).expect("Failed to read password from file");
                let (nonce_bytes, encrypted_password) = data.split_at(secretbox::NONCEBYTES);
                let nonce = secretbox::Nonce::from_slice(nonce_bytes).expect("Invalid nonce length");
                match open(&encrypted_password, &nonce, &key) {
                    Ok(decoded_password) => {
                        let password = String::from_utf8(decoded_password).expect("Failed to convert password to string");
                        println!("{}", password);
                    }
                    Err(_) => {
                        println!("{}", "Failed to decrypt password".red());
                    }
                }
            } else {
                println!("No such file: {}", name.red());
            }
        }
        "-v" | "--version" => {
            println!("{}", format!("QPass v{}", VERSION).blue());
        }
        _ => {
            usage();
            std::process::exit(1);
        }
    }
}
