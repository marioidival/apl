use crate::primitive::Primitive;

#[derive(Debug)]
pub enum Error {
    InvalidOperation(OperatorError, Primitive, Option<Primitive>),
}


#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    Message(String),
}

#[derive(Debug)]
pub enum OperatorError {
    Add,
    Sub,
    Mul,
    IntDiv,
    RealDiv,
    UnarySub,
    Negate,
    And,
    Or,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
}