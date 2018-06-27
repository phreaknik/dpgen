#[macro_use]
extern crate clap;
extern crate rpassword;
extern crate sha3;

use clap::App;
use sha3::{Digest, Sha3_256};
use std::io::{self, Write};
use std::process;

fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .get_matches();

    // Print welcome message
    println!("dpgen - A deterministic password generator\n");
    println!(
        "Generate strong passwords for all your logins, \
         and only remember one 'master password' to keep \
         them all secure.\n"
    );

    // Get inputs
    let url = get_input("Website URL: ").unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });
    let username = get_input("Email / Username: ").unwrap_or_else(|err| {
        println!("error: {}", err);
        process::exit(1);
    });

    // Prompt for a password
    println!("\nNow enter your 'master password'.");
    println!(
        "Warning: Be careful not to make any typos! You will \
         need to type the EXACT same answers and 'master \
         password' to recover the same password later!\n"
    );
    let master_pass = rpassword::prompt_password_stdout("Master password: ").unwrap();

    // Concatenate inputs to create message to hash
    let mut msg = "PASSWORD SEED = ".to_owned();
    msg.push_str(&url.to_string());
    msg.push_str(&username.to_string());
    msg.push_str(&master_pass.to_string());

    // Hash the message
    let mut hasher = Sha3_256::default();
    hasher.input(msg.as_bytes());
    let out = hasher.result();

    println!("{:x}", out);
}

fn get_input(msg: &str) -> (Result<String, String>) {
    // Print message
    print!("{}", msg);
    io::stdout().flush().unwrap();

    // Read input
    let mut s = String::new();
    if let Err(error) = io::stdin().read_line(&mut s) {
        return Err(error.to_string());
    }

    // Strip off newline or carriage return characters
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    // Make sure input is non-empty
    if s.len() == 0 {
        return Err("No input detected.".to_string());
    } else {
        return Ok(s);
    }
}
