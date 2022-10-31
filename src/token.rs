use fancy_regex::Regex;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct InvalidTokenErr {}

impl Display for InvalidTokenErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidTokenErr")
    }
}

impl Error for InvalidTokenErr {}

#[derive(Debug)]
pub enum Token {
    Operator(String),
    LiteralStr(String),
    LiteralNum(String),
    Identifier(String),
    ScopeIn(String),
    ScopeOut(String),
    StructStart(String),
    StructEnd(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

type NewToken = fn(String) -> Token;

static PATTERNS: &[(&[&str], NewToken)] = &[
    (
        &[
            "\\+", "\\-", "\\*", "\\/", "=", "!", "\\?", "\\|", "&", "==", "!=", "\\<=", "\\>=",
            "\\<", "\\>", "\\^", "@", "#", "~",
        ],
        Token::Operator,
    ),
    (&["\"(?s:.*)(?<!\\\\)\""], Token::LiteralStr),
    (&["\\d+"], Token::LiteralNum),
    (&["\\("], Token::ScopeIn),
    (&["\\)"], Token::ScopeOut),
    (&["\\{"], Token::StructStart),
    (&["\\}"], Token::StructEnd),
    (&[".+"], Token::Identifier),
];

impl FromStr for Token {
    type Err = InvalidTokenErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for (pattern, new_token) in PATTERNS {
            let str = &*format!("^(?<!\\\\)({})$", pattern.join("|"));
            let regex = Regex::new(str).unwrap();

            if regex.is_match(s).unwrap_or(false) {
                return Ok(new_token(String::from(s)));
            }
        }

        Err(InvalidTokenErr {})
    }
}
