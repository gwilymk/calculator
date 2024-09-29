use std::collections::HashMap;

use crate::{
    ast::{Expression, ExpressionKind, Operator, Statement, StatementKind},
    reporting,
};

pub fn execute(ast: Vec<Statement<'_>>) -> Result<(), reporting::Message> {
    let mut variables = HashMap::new();

    for statement in ast {
        match statement.kind {
            StatementKind::Variable { name, value } => {
                variables.insert(name, exec_expression(&value, &variables)?);
            }
            StatementKind::Print { value } => {
                println!("{}", exec_expression(&value, &variables)?);
            }
            StatementKind::Error(e) => return Err(e),
        }
    }

    Ok(())
}

fn exec_expression<'input>(
    expression: &Expression<'input>,
    variables: &HashMap<&'input str, i64>,
) -> Result<i64, reporting::Message> {
    match &expression.kind {
        ExpressionKind::Integer(i) => Ok(*i),
        ExpressionKind::Variable(name) => {
            variables
                .get(name)
                .copied()
                .ok_or(reporting::Message::UnknownVariable(
                    name.to_string(),
                    expression.location,
                ))
        }
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
        ExpressionKind::Error(e) => Err(e.clone()),
    }
}
