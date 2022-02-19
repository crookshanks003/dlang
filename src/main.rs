mod lexer;
use std::{env, fs, process};

use lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut code = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut lexer = Lexer::new(&mut code);
    let tokens = lexer.get_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
}

fn parse_args(args: &Vec<String>) -> Result<String, &str> {
    if args.len() < 2 {
        Err("No filename provided. use: dlang <filename>")
    } else if args[1] == "test" {
        let test = "int fact( int n) {
if (n << 1)
return 1;
else";
        Ok(test.to_string())
    } else {
        let filename = &args[1];
        match fs::read_to_string(filename) {
            Ok(x) => Ok(x),
            Err(_) => Err("No such file exist"),
        }
    }
}
