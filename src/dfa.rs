use dlang::CharacterType;



pub struct Dfa {
    curr_state: i32,
    prev_state: i32,
}

impl Dfa {
    pub fn new() -> Self {
        Dfa {
            curr_state: 0,
            prev_state: 0,
        }
    }

    pub fn get_character_type(ch: char) -> CharacterType {
        match ch {
            'A'..='Z' | 'a'..='z' | '_' => CharacterType::Alphabet,
            '0'..='9' => CharacterType::Num,
            ' ' | '\n' | '\t' => CharacterType::Whitespace,
            '+' | '-' | '*' | '/' | '%' => CharacterType::Arithmatic,
            '=' => CharacterType::Equal,
            '>' | '<' | '!' => CharacterType::Logic,
            '{' | '}' | '[' | ']' | ';' | ',' | '(' | ')' => CharacterType::Deliminator,
            '&' => CharacterType::And,
            '|' => CharacterType::Or,
            '"' => CharacterType::Quote,
            '$' => CharacterType::EOF,
            '.' => CharacterType::Dot,
            _ => CharacterType::Invalid,
        }
    }
}
