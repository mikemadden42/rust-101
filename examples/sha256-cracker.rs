use sha2::{Digest, Sha256};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid arguments!");
        println!(">> {} <sha256sum>", args[0]);
        exit(1);
    }

    let wanted_hash = &args[1];
    let password_file = "src/rockyou.txt";

    println!("Attempting to crack: {wanted_hash}!\n");

    let password_list = match File::open(password_file) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening wordlist '{password_file}': {err}");
            eprintln!("Place a wordlist (e.g. rockyou.txt) at that path and try again.");
            exit(1);
        }
    };
    let reader = BufReader::new(password_list);

    for (attempts, line) in (1..).zip(reader.lines()) {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Error reading wordlist: {err}");
                continue;
            }
        };
        let password = line.trim();
        let password_hash = format!("{:x}", Sha256::digest(password.as_bytes()));
        if &password_hash == wanted_hash {
            println!("Password hash found after {attempts} attempts! {password} hashes to {password_hash}!");
            exit(0);
        }
    }

    println!("Password hash not found!");
}
