use std::io;

enum TokenType {
    Nat,
    Plus,
    Minus,
    Times,
    LeftParenthesis,
    RightParenthesis,
}

struct Token {
    pos: usize,
    len: usize,
    token_type: TokenType,
    token_value: Option<u32>,
}

fn strtol(x: &Vec<char>, offset: usize) -> (u32, usize) {
    let mut n = offset;
    let mut r: u32 = 0;
    while n < x.len() && x[n].is_digit(10) {
        r = r * 10 + x[n].to_digit(10).unwrap();
        n = n + 1;
    }
    (r, n - offset)
}

fn tokenize(input_str: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let chars = input_str.chars().collect::<Vec<char>>();
    let mut i: usize = 0;
    let l = chars.len();
    while i < l {
        let x = chars[i];
        match x {
            ' ' => {
                i = i + 1;
            },
            '(' => {
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    token_type: TokenType::LeftParenthesis,
                    token_value: Option::None,
                });
                i = i + 1;
            },
            ')' => {
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    token_type: TokenType::RightParenthesis,
                    token_value: Option::None,
                });
                i = i + 1;
            },
            '+' => {
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    token_type: TokenType::Plus,
                    token_value: Option::None,
                });
                i = i + 1;
            },
            '-' => {
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    token_type: TokenType::Minus,
                    token_value: Option::None,
                });
                i = i + 1;
            },
            '*' => {
                tokens.push(Token {
                    pos: i,
                    len: 1,
                    token_type: TokenType::Times,
                    token_value: Option::None,
                });
                i = i + 1;
            },
            _ => {
                let (v, x) = strtol(&chars, i);
                if x == 0 { 
                    break;
                }
                tokens.push(Token {
                    pos: i,
                    len: x,
                    token_type: TokenType::Nat,
                    token_value: Option::Some(v),
                });
                i = i + x;
            }
        }
    }
    tokens
}

fn main() {
    let mut input_str = String::new();
    io::stdin().read_line(&mut input_str).expect("fail");

    let tokens = tokenize(input_str);

    for t in tokens {
        print!("[{}:{}] ", t.pos, t.len);
        match t.token_type {
            TokenType::LeftParenthesis => println!("LeftParenthesis"),
            TokenType::RightParenthesis => println!("RightParenthesis"),
            TokenType::Plus => println!("Plus"),
            TokenType::Minus => println!("Minus"),
            TokenType::Times => println!("Times"),
            TokenType::Nat => println!("Nat: {} ", t.token_value.unwrap()),
        }
    }
}
