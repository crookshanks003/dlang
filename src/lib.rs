pub mod dfa;

#[derive(Clone, Copy)]
pub enum CharacterType {
    Invalid = -1,
    Alphabet,
    Num,
    Whitespace,
    Arithmatic,
    Equal,
    Logic,
    Deliminator,
    And,
    Or,
    Quote,
    Dot,
    EOF,
}

pub const KEYWORDS: [&str; 12] = [
    "int", "float", "string", "bool", "if", "else", "elif", "while", "for", "break", "true",
    "false",
];

pub fn check_if_keyword(lexeme: &str) -> bool {
    for keyword in KEYWORDS {
        if lexeme == keyword {
            return true;
        }
    }
    false
}

/* DFA_TABLE
 *    char    alpha  num  whi  ari  eq  log  del  and  or  quo  dot  eof
 * 0  start     1     2    0    4   5    6    8   9    11  14   13   15
 * 1  alpha     1     1    0    0   0    0    0   0    0   0    -1   0
 * 2  num       1     2    0    0   0    0    0   0    0   0    3    0
 * 3  flo       0     3    0    0   0    0    0   0    0   0    -1   0
 * 4  ari       0     0    0    0   0    0    0   0    0   0    -1   0
 * 5  eq        0     0    0    0   6    0    0   0    0   0    -1   0
 * 6  log       0     0    0    0   7    7    0   0    0   0    -1   0
 * 7  log2      0     0    0    0   0    0    0   0    0   0    -1   0
 * 8  del       0     0    0    0   0    0    0   0    0   0    -1   0
 * 9  and       -1    -1   -1   -1  -1   -1   -1  10   -1  -1   -1   -1
 * 10 and2      0     0    0    0   0    0    0   0    0   0    -1   0
 * 11 or        -1    -1   -1   -1  -1   -1   -1  -1   12  -1   -1   -1
 * 12 or2       0     0    0    0   0    0    0   0    0   0    -1   0
 * 13 dot       -1    3   -1   -1  -1   -1   -1  -1   -1   -1   -1   -1
 * */

pub const DFA_TABLE: [[i32; 12]; 15] = [
    [1, 2, 0, 4, 5, 6, 8, 9, 11, 14, 13, 15],
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0],
    [0, 3, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [0, 0, 0, 0, 6, 0, 0, 0, 0, 0, -1, 0],
    [0, 0, 0, 0, 7, 7, 0, 0, 0, 0, -1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [-1, -1, -1, -1, -1, -1, -1, 10, -1, -1, -1, -1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [-1, -1, -1, -1, -1, -1, -1, -1, 12, -1, -1, -1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0],
    [-1, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
];
