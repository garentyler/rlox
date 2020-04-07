use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            run_prompt();
        },
        2 => {
            run_file(args.get(1).unwrap());
        },
        _ => {
            println!("Usage: rlox [script]");
        }
    }
}
fn run_prompt() {
    println!("Welcome to the rlox REPL");
    loop {
        print!("> "); // Display the prompt
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_bytes_read) => {
                input.pop(); // Remove trailing newline
                std::io::stdout().flush().unwrap();
                match rlox::run(&input) {
                    Ok(_) => {},
                    Err(err) => {
                        println!("error: {}", err);
                    }
                };
            }
            Err(err) => println!("error reading input: {}", err),
        }
    }
}
fn run_file(filename: &str) {
    if !std::path::Path::new(&filename).exists() {
        panic!(format!("Error running rlox file: {} does not exist", filename));
    }
    let mut file = File::open(&filename)
        .expect(&format!("Error running rlox file: Could not open {}", filename));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error running rlox file: Could not copy to string");
    match rlox::run(&contents) {
        Ok(_) => { // Exit cleanly.
            std::process::exit(0);
        },
        Err(_err) => { // Exit with error.
            std::process::exit(1);
        }
    };
}
