use crate::primitive::Primitive;
use crate::object::Object;

#[derive(Debug)]
pub enum Error {
    InvalidOperation(OperatorError, Primitive, Option<Primitive>),
    InvalidType(OperatorError, Object, Option<Object>),
    OtherError(String),
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
    UnaryPlus,
    Negate,
    And,
    Or,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    Is,
    NotIs
}