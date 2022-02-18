mod lexer;
mod dfa;
use std::{env, fs, process};

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let lexer = Lexer::new(code);

}

fn parse_args(args: &Vec<String>) -> Result<String, &str> {
    if args.len() < 2 {
        Err("No filename provided. use: dlang <filename>")
    } else if args[1] == "test" {
        Ok("int fact( int n)".to_string())
    } else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(x) => Ok(x),
            Err(_) => Err("No such file exist"),
        }
    }
}
