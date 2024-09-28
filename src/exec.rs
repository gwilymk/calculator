use std::{collections::HashMap, fmt};

use crate::ast::{Expression, Operator, Statement};

#[derive(Debug)]
pub enum ExecutionError {
    UnknownVariable(String),
    ExpressionError,
    StatementError,
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::UnknownVariable(v) => write!(f, "Unknown variable {v}"),
            ExecutionError::ExpressionError => {
                write!(f, "Expression error (this should never happen)")
            }
            ExecutionError::StatementError => {
                write!(f, "Statement error (this should never happen)")
            }
        }
    }
}

pub fn execute(ast: Vec<Statement>) -> Result<(), ExecutionError> {
    let mut variables = HashMap::new();

    for statement in ast {
        match statement {
            Statement::Variable { name, value } => {
                variables.insert(name, exec_expression(&value, &variables)?);
            }
            Statement::Print { value } => {
                println!("{}", exec_expression(&value, &variables)?);
            }
            Statement::Error => return Err(ExecutionError::StatementError),
        }
    }

    Ok(())
}

fn exec_expression<'input>(
    expression: &Expression<'input>,
    variables: &HashMap<&'input str, i64>,
) -> Result<i64, ExecutionError> {
    match expression {
        Expression::Integer(i) => Ok(*i),
        Expression::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| ExecutionError::UnknownVariable(name.to_string())),
        Expression::BinaryOperation { lhs, operator, rhs } => {
            let lhs = exec_expression(lhs, variables)?;
            let rhs = exec_expression(rhs, variables)?;

            Ok(match operator {
                Operator::Add => lhs + rhs,
                Operator::Sub => lhs - rhs,
                Operator::Mul => lhs * rhs,
                Operator::Div => lhs / rhs,
            })
        }
        Expression::Error => Err(ExecutionError::ExpressionError),
    }
}
