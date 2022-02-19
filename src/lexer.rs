use plang::{check_if_keyword, dfa::Dfa};
use std::{collections::HashSet, process};

pub struct Lexer {
    code: Vec<char>,
    dfa: Dfa,
}

#[derive(Debug)]
pub struct Token<'a> {
    token: &'a str,
    lexeme: String,
    line: u32,
}

impl Lexer {
    pub fn new(code: &mut String) -> Self {
        code.push('$');
        let dfa = Dfa::new();
        Lexer {
            code: code.chars().collect(),
            dfa,
        }
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut sp: usize = 0;
        let mut fp: usize = 0;
        let mut line: u32 = 0;

        loop {
            let ch = self.code[fp];

            self.dfa.transition_state(ch);

            if self.dfa.curr_state == 0 && self.dfa.prev_state == 0 {
                sp += 1;
                fp += 1;
                if ch == '\n' {
                    line += 1;
                }
                continue;
            }

            if ch == '/' && self.code[fp + 1] == '*' {
                match self.handle_comments(&mut fp, &mut line) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("{} at `{}` line: {}", err, ch, line);
                        process::exit(1);
                    }
                }
                fp += 1;
                sp = fp;
                self.dfa.curr_state = 0;
                continue;
            }

            if self.dfa.curr_state == 0 {
                let lexeme = String::from_iter(self.code[sp..fp].iter());
                let token = match self.dfa.prev_state {
                    1 if check_if_keyword(&lexeme) => "keyword",
                    1 => "id",
                    2 => "int",
                    3 => "float",
                    4 | 5 | 6 | 7 | 10 | 12 => "op",
                    8 => "del",
                    14 => "str",
                    _ => unreachable!(),
                };

                tokens.push(Token {
                    token,
                    lexeme,
                    line,
                });

                sp = fp;
                fp -= 1;
            }

            if ch == '"' {
                fp += 1;
                match self.handle_string(&mut fp) {
                    Ok(_) => (),
                    Err(err) => {
                        eprintln!("{} at `{}` line: {}", err, ch, line);
                        process::exit(1);
                    }
                };
            } else if ch == '$' {
                break;
            } 

            if self.dfa.curr_state == -1 {
                eprintln!("Invalid char at `{}`, line: {}", ch, line);
                process::exit(1);
            }

            fp += 1;
        }
        tokens
    }

    fn handle_string(&mut self, fp: &mut usize) -> Result<(), &str> {
        let escape_chars: HashSet<char> = HashSet::from(['n', 't', 'r', '\\', '"']);
        loop {
            *fp += 1;
            if self.code[*fp] == '"' {
                break;
            } else if self.code[*fp] == '\\' && !escape_chars.contains(&self.code[*fp + 1]) {
                return Err("Bad escape sequence");
            } else if self.code[*fp] == '$' || self.code[*fp] == '\n' {
                return Err("Unterminated string literal");
            }
        }
        self.dfa.curr_state = 14;
        Ok(())
    }

    fn handle_comments(&mut self, fp: &mut usize, line: &mut u32) -> Result<(), &str> {
        loop {
            *fp += 1;
            if self.code[*fp] == '\n' {
                *line += 1;
            }
            if self.code[*fp] == '*' && self.code[*fp + 1] == '/' {
                *fp += 1;
                break;
            } else if self.code[*fp] == '$' {
                return Err("Unterminated comment block");
            }
        }
        Ok(())
    }
}
