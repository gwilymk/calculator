use std::ops::Range;

use lalrpop_util::{ErrorRecovery, ParseError};
use serde::{Serialize, Serializer};

use crate::tokens::{self, LexicalError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Location(pub usize, pub usize);

impl Location {
    pub fn as_range(self) -> Range<usize> {
        self.0..self.1
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Statement<'input> {
    pub location: Location,
    pub kind: StatementKind<'input>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum StatementKind<'input> {
    Variable {
        name: &'input str,
        value: Box<Expression<'input>>,
    },
    Print {
        value: Box<Expression<'input>>,
    },
    Error(
        #[serde(serialize_with = "serialize_user_error_only")]
        ErrorRecovery<usize, tokens::Token<'input>, LexicalError>,
    ),
}

impl<'input> StatementKind<'input> {
    pub fn with_loc(self, location: Location) -> Statement<'input> {
        Statement {
            kind: self,
            location,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Expression<'input> {
    pub location: Location,
    pub kind: ExpressionKind<'input>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum ExpressionKind<'input> {
    Integer(i64),
    Variable(&'input str),
    BinaryOperation {
        lhs: Box<Expression<'input>>,
        operator: Operator,
        rhs: Box<Expression<'input>>,
    },
    Error(
        #[serde(serialize_with = "serialize_user_error_only")]
        ErrorRecovery<usize, tokens::Token<'input>, LexicalError>,
    ),
}

impl<'input> ExpressionKind<'input> {
    pub fn with_loc(self, location: Location) -> Box<Expression<'input>> {
        Box::new(Expression {
            kind: self,
            location,
        })
    }
}

impl<'input> From<LexicalError> for ExpressionKind<'input> {
    fn from(value: LexicalError) -> Self {
        Self::Error(ErrorRecovery {
            error: value.into(),
            dropped_tokens: vec![],
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

fn serialize_user_error_only<S>(
    error_recovery: &ErrorRecovery<usize, tokens::Token<'_>, LexicalError>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let maybe_value = if let ParseError::User { error } = &error_recovery.error {
        Some(error)
    } else {
        None
    };

    maybe_value.serialize(serializer)
}
