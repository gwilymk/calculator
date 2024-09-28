use std::ops::Range;

use lalrpop_util::ErrorRecovery;

use crate::tokens::{self, LexicalError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location(pub usize, pub usize);

impl Location {
    pub fn as_range(self) -> Range<usize> {
        self.0..self.1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement<'input> {
    pub location: Location,
    pub kind: StatementKind<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum StatementKind<'input> {
    Variable {
        name: &'input str,
        value: Box<Expression<'input>>,
    },
    Print {
        value: Box<Expression<'input>>,
    },
    Error(ErrorRecovery<usize, tokens::Token<'input>, LexicalError>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expression<'input> {
    pub location: Location,
    pub kind: ExpressionKind<'input>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExpressionKind<'input> {
    Integer(i64),
    Variable(&'input str),
    BinaryOperation {
        lhs: Box<Expression<'input>>,
        operator: Operator,
        rhs: Box<Expression<'input>>,
    },
    Error(ErrorRecovery<usize, tokens::Token<'input>, LexicalError>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
