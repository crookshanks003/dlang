pub struct Lexer {
    code: String,
}

struct Token<'a> {
    token: &'a str,
    lexeme: &'a str,
    line: u32
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Lexer { code }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        tokens
    }
}
