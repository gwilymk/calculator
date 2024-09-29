use std::ops::Range;

use serde::Serialize;

use crate::reporting;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Default)]
pub struct Location(pub usize, pub usize);

impl Location {
    pub fn as_range(self) -> Range<usize> {
        self.0..self.1
    }

    pub fn unwrap_or(self, other: Self) -> Self {
        if self.0 == 0 && self.1 == 0 {
            other
        } else {
            self
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Statement<'input> {
    pub location: Location,
    pub kind: StatementKind<'input>,
}

#[derive(Clone, Debug, Serialize)]
pub enum StatementKind<'input> {
    Variable {
        name: &'input str,
        value: Box<Expression<'input>>,
    },
    Print {
        value: Box<Expression<'input>>,
    },
    Error(reporting::Message),
}

impl<'input> StatementKind<'input> {
    pub fn with_loc(self, location: Location) -> Statement<'input> {
        Statement {
            kind: self,
            location,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Expression<'input> {
    pub location: Location,
    pub kind: ExpressionKind<'input>,
}

#[derive(Clone, Debug, Serialize)]
pub enum ExpressionKind<'input> {
    Integer(i64),
    Variable(&'input str),
    BinaryOperation {
        lhs: Box<Expression<'input>>,
        operator: Operator,
        rhs: Box<Expression<'input>>,
    },
    Error(reporting::Message),
}

impl<'input> ExpressionKind<'input> {
    pub fn with_loc(self, location: Location) -> Box<Expression<'input>> {
        Box::new(Expression {
            kind: self,
            location,
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
