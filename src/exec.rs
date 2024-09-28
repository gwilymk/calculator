use std::{collections::HashMap, fmt};

use crate::ast::{Expression, ExpressionKind, Location, Operator, Statement, StatementKind};

#[derive(Debug)]
pub enum ExecutionError {
    UnknownVariable(String, Location),
    ExpressionError(Location),
    StatementError(Location),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::UnknownVariable(v, _) => write!(f, "Unknown variable {v}"),
            ExecutionError::ExpressionError(_) => {
                write!(f, "Expression error (this should never happen)")
            }
            ExecutionError::StatementError(_) => {
                write!(f, "Statement error (this should never happen)")
            }
        }
    }
}

impl ExecutionError {
    pub fn location(&self) -> Location {
        match self {
            ExecutionError::UnknownVariable(_, location)
            | ExecutionError::ExpressionError(location)
            | ExecutionError::StatementError(location) => *location,
        }
    }
}

pub fn execute(ast: Vec<Statement>) -> Result<(), ExecutionError> {
    let mut variables = HashMap::new();

    for statement in ast {
        match statement.kind {
            StatementKind::Variable { name, value } => {
                variables.insert(name, exec_expression(&value, &variables)?);
            }
            StatementKind::Print { value } => {
                println!("{}", exec_expression(&value, &variables)?);
            }
            StatementKind::Error => return Err(ExecutionError::StatementError(statement.location)),
        }
    }

    Ok(())
}

fn exec_expression<'input>(
    expression: &Expression<'input>,
    variables: &HashMap<&'input str, i64>,
) -> Result<i64, ExecutionError> {
    match &expression.kind {
        ExpressionKind::Integer(i) => Ok(*i),
        ExpressionKind::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| ExecutionError::UnknownVariable(name.to_string(), expression.location)),
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
        ExpressionKind::Error => Err(ExecutionError::ExpressionError(expression.location)),
    }
}
