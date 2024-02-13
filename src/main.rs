mod errors;
use errors::error;
use scanner::*;

fn run_file(filename: &str) {
    let file = std::fs::read_to_string(filename).expect("Error reading file");
    let source = file.as_str();
    run(source);
}
fn run_prompt() {
    println!("Welcome to the Lox(rs) interpreter");
    loop {
        print!("> ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        run(input.as_str());
    }
}
fn run(source: &str) {
    let scanner = Scanner::new(source);
    println!("{}", source);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        run_file(&args[1]);
    } else if args.len() == 1 {
        run_prompt();
    } else {
        println!("Usage: cargo run <filename> OR cargo run to enter interactive mode");
    }
}
