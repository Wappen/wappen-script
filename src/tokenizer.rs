use std::str::FromStr;

use crate::Token;

pub fn tokenize(code: &str) -> Vec<Token> {
    let words = split(code);
    let mut tokens = vec![];

    for word in words {
        let token = Token::from_str(&*word).unwrap();
        tokens.push(token);
    }

    tokens
}

fn split(code: &str) -> Vec<String> {
    let mut words = Vec::new();

    let mut in_quotes = false;
    let mut escaped = false;

    let mut current_word = String::new();

    for c in code.chars() {
        match c {
            '"' => {
                if !escaped {
                    in_quotes ^= true;
                }
            }
            '\\' => {
                if !escaped {
                    escaped = true;
                    continue;
                }
            }
            ' ' | '\r' | '\n' | '\t' => {
                if !escaped && !in_quotes {
                    let new_word = String::from(current_word.clone().trim());
                    if !new_word.is_empty() {
                        words.push(new_word);
                    }
                    current_word = String::new();
                    continue;
                }
            }
            _ => {}
        }

        current_word.push(c);
        escaped = false;
    }

    words
}
