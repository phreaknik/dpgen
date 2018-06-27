#[macro_use]
extern crate clap;
extern crate secp256k1;

use clap::App;
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
        "Please answer the following questions. Your \
        answers will be used to generate a strong \
        password. No need to remember your password, \
        you can always type the same answers into \
        this tool later to see what your password is. \
        \n\nWarning: Be careful not to make any typos! You will \
        need to type the EXACT same answers if you \
        want to regenerate the same password later!\n"
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

    println!("{}@{}", username, url);
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
