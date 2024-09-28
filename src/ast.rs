#[derive(Clone, Debug, PartialEq)]
pub enum Statement<'input> {
    Variable {
        name: &'input str,
        value: Box<Expression<'input>>,
    },
    Print {
        value: Box<Expression<'input>>,
    },
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression<'input> {
    Integer(i64),
    Variable(&'input str),
    BinaryOperation {
        lhs: Box<Expression<'input>>,
        operator: Operator,
        rhs: Box<Expression<'input>>,
    },
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}
