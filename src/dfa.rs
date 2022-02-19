use crate::{CharacterType, DFA_TABLE};

pub struct Dfa {
    pub curr_state: isize,
    pub prev_state: isize,
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

    pub fn transition_state(&mut self, ch: char) {
        self.prev_state = self.curr_state;
        let char_type = Dfa::get_character_type(ch);
        if let CharacterType::Invalid = char_type {
            self.curr_state = -1;
        } else {
            self.curr_state = DFA_TABLE[self.curr_state as usize][char_type as usize] as isize;
        }
    }
}
