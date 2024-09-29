use std::fmt;
use std::num::ParseIntError;

use logos::Logos;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub enum LexicalError {
    InvaidInteger(#[serde(skip)] ParseIntError),
    #[default]
    InvalidToken,
}

impl From<ParseIntError> for LexicalError {
    fn from(value: ParseIntError) -> Self {
        LexicalError::InvaidInteger(value)
    }
}

#[derive(Logos, Clone, Debug, PartialEq, Serialize)]
#[logos(skip r"[ \t\n\f]+", skip r"#.*\n?", error = LexicalError)]
pub enum Token<'input> {
    #[token("var")]
    KeywordVar,
    #[token("print")]
    KeywordPrint,

    #[regex("[_a-zA-Z][_0-9a-zA-Z]*", |lex| lex.slice())]
    Identifier(&'input str),
    #[regex("-?[0-9]+", |lex| lex.slice())]
    Integer(&'input str),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Assign,
    #[token(";")]
    Semicolon,

    #[token("+")]
    OperatorAdd,
    #[token("-")]
    OperatorSub,
    #[token("*")]
    OperatorMul,
    #[token("/")]
    OperatorDiv,
}

impl<'input> fmt::Display for Token<'input> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvaidInteger(parse_int_error) => write!(f, "{parse_int_error}"),
            LexicalError::InvalidToken => write!(f, "Invalid token"),
        }
    }
}
