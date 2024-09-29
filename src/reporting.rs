use crate::{
    ast::Location,
    tokens::{self, LexicalError},
};

mod format;

pub use format::WriteDiagnostic;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum Message {
    LexerError(Box<LexicalError>, Location),
    ParseError(Box<ParseError>),
    UnknownVariable(String, Location),
}

impl Message {
    pub fn lexer_error(err: LexicalError, location: Location) -> Self {
        Self::LexerError(Box::new(err), location)
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum ParseError {
    UnrecognizedEof {
        location: Location,
        expected: Vec<String>,
    },
    UnrecognisedToken {
        location: Location,
        token: String,
        expected: Vec<String>,
    },
    ExtraToken {
        location: Location,
        token: String,
    },
}

impl Message {
    pub fn from_lalrpop(
        value: lalrpop_util::ParseError<usize, tokens::Token<'_>, LexicalError>,
        location_override: Option<Location>,
    ) -> Self {
        match value {
            lalrpop_util::ParseError::InvalidToken { location } => Message::lexer_error(
                LexicalError::InvalidToken,
                location_override.unwrap_or(Location(location, location)),
            ),
            lalrpop_util::ParseError::UnrecognizedEof { location, expected } => {
                Message::ParseError(Box::new(ParseError::UnrecognizedEof {
                    location: location_override.unwrap_or(Location(location, location)),
                    expected,
                }))
            }
            lalrpop_util::ParseError::UnrecognizedToken { token, expected } => {
                Message::ParseError(Box::new(ParseError::UnrecognisedToken {
                    location: location_override.unwrap_or(Location(token.0, token.2)),
                    token: format!("{}", token.1),
                    expected,
                }))
            }
            lalrpop_util::ParseError::ExtraToken { token } => {
                Message::ParseError(Box::new(ParseError::ExtraToken {
                    location: location_override.unwrap_or(Location(token.0, token.2)),
                    token: format!("{}", token.1),
                }))
            }
            lalrpop_util::ParseError::User { error } => {
                Message::lexer_error(error, location_override.unwrap_or_default())
            }
        }
    }
}
