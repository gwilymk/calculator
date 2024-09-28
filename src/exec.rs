use std::collections::HashMap;

use lalrpop_util::ErrorRecovery;

use crate::{
    ast::{Expression, ExpressionKind, Location, Operator, Statement, StatementKind},
    tokens::{self, LexicalError},
};

#[derive(Debug)]
pub enum ExecutionError<'input> {
    UnknownVariable(&'input str, Location),
    ExpressionError(
        ErrorRecovery<usize, tokens::Token<'input>, LexicalError>,
        Location,
    ),
    StatementError(
        ErrorRecovery<usize, tokens::Token<'input>, LexicalError>,
        Location,
    ),
}

impl<'input> ExecutionError<'input> {
    pub fn location(&self) -> Location {
        match self {
            ExecutionError::UnknownVariable(_, location)
            | ExecutionError::ExpressionError(_, location)
            | ExecutionError::StatementError(_, location) => *location,
        }
    }

    pub fn error_recovery(
        &self,
    ) -> Option<ErrorRecovery<usize, tokens::Token<'input>, LexicalError>> {
        match self {
            ExecutionError::UnknownVariable(..) => None,
            ExecutionError::ExpressionError(error_recovery, _)
            | ExecutionError::StatementError(error_recovery, _) => Some(error_recovery.clone()),
        }
    }

    pub fn message(&self) -> Option<String> {
        match self {
            ExecutionError::UnknownVariable(var_name, _) => {
                Some(format!("Unknown variable {var_name}"))
            }
            _ => None,
        }
    }
}

pub fn execute(ast: Vec<Statement<'_>>) -> Result<(), ExecutionError<'_>> {
    let mut variables = HashMap::new();

    for statement in ast {
        match statement.kind {
            StatementKind::Variable { name, value } => {
                variables.insert(name, exec_expression(&value, &variables)?);
            }
            StatementKind::Print { value } => {
                println!("{}", exec_expression(&value, &variables)?);
            }
            StatementKind::Error(e) => {
                return Err(ExecutionError::StatementError(e, statement.location))
            }
        }
    }

    Ok(())
}

fn exec_expression<'input>(
    expression: &Expression<'input>,
    variables: &HashMap<&'input str, i64>,
) -> Result<i64, ExecutionError<'input>> {
    match &expression.kind {
        ExpressionKind::Integer(i) => Ok(*i),
        ExpressionKind::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or(ExecutionError::UnknownVariable(name, expression.location)),
        ExpressionKind::BinaryOperation { lhs, operator, rhs } => {
            let lhs = exec_expression(lhs, variables)?;
            let rhs = exec_expression(rhs, variables)?;

            Ok(match operator {
                Operator::Add => lhs + rhs,
                Operator::Sub => lhs - rhs,
                Operator::Mul => lhs * rhs,
                Operator::Div => lhs / rhs,
            })
        }
        ExpressionKind::Error(e) => Err(ExecutionError::ExpressionError(
            e.clone(),
            expression.location,
        )),
    }
}
