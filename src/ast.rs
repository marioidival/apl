use crate::ast::Comparison::In;
use crate::ast::Expression::{BoolOp, Compare};
use crate::token::Token;

use self::Primitive::*;

#[derive(Debug, PartialEq)]
pub enum UnaryOperation {
    Minus,
    Not,
}

impl From<Token> for UnaryOperation {
    fn from(tk: Token) -> Self {
        match tk {
            Token::Nao => UnaryOperation::Not,
            Token::Minus => UnaryOperation::Minus,
            _ => panic!("ins't token for boolean operation!")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or,
}

impl From<Token> for BooleanOperation {
    fn from(tk: Token) -> Self {
        match tk {
            Token::E => BooleanOperation::And,
            Token::Ou => BooleanOperation::Or,
            _ => panic!("ins't token for boolean operation!")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl From<Token> for Operator {
    fn from(tk: Token) -> Self {
        match tk {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Sub,
            Token::Slash => Operator::Div,
            Token::Star => Operator::Mul,
            Token::Percent => Operator::Mod,
            _ => panic!("unit type isn't a real type")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Greater,
    GreaterThan,
    Less,
    LessThan,
    In,
    NotIn,
    Is,
    NotIs,
}

impl From<Token> for Comparison {
    fn from(tk: Token) -> Self {
        match tk {
            Token::Less => Comparison::Less,
            Token::LessThan => Comparison::LessThan,
            Token::Greater => Comparison::Greater,
            Token::GreaterThan => Comparison::GreaterThan,
            Token::EqualEqual => Comparison::Equal,
            Token::BangEqual => Comparison::NotEqual,
            _ => panic!("unit type isn't a real type")
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: i64 },
    Float { value: f64 },
}

#[derive(Debug, PartialEq)]
pub struct Keyword {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Compare {
        a: Box<Expression>,
        op: Comparison,
        b: Box<Expression>,
    },
    BoolOp {
        a: Box<Expression>,
        op: BooleanOperation,
        b: Box<Expression>,
    },
    BinOp {
        a: Box<Expression>,
        op: Operator,
        b: Box<Expression>,
    },
    UnOp {
        op: UnaryOperation,
        a: Box<Expression>,
    },
    Str {
        value: String,
    },
    Num {
        value: Number,
    },
    IfExpression {
        test: Box<Expression>,
        body: Box<Expression>,
        orelse: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
        keywords: Vec<Keyword>,
    },
    Identifier {
        name: String,
    },
    True,
    False,
    None,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
//    FunctionDef,
//    ClassDef,
//    Delete {},
//    For,
//    While,
//    If,
//    Raise,
//    Try,

    Break,
    Continue,
    Pass,
    Return {
        value: Option<Vec<Expression>>
    },
    Assert {
        test: Expression,
        msg: Option<Expression>,
    },
    Assign {
        targets: Vec<Expression>,
        value: Expression,
    },
    Expr {
        expression: Expression,
    },
}

#[derive(Debug, PartialEq)]
pub enum Top {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

// move to other file
#[derive(Debug)]
pub enum Error {
    InvalidOperation(OperatorError, Primitive, Option<Primitive>),
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

#[derive(Debug)]
pub enum Object {
    Primitive(Primitive),
}

impl From<i64> for Object {
    fn from(n: i64) -> Self {
        Object::Primitive(Primitive::Integer(n))
    }
}

impl From<f64> for Object {
    fn from(n: f64) -> Self {
        Object::Primitive(Primitive::Float(n))
    }
}

impl From<String> for Object {
    fn from(n: String) -> Self {
        Object::Primitive(Primitive::Str(n))
    }
}

impl From<bool> for Object {
    fn from(n: bool) -> Self {
        Object::Primitive(Primitive::Boolean(n))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    Integer(i64),
    Float(f64),
    Str(String),
    Boolean(bool),
}

impl From<i64> for Primitive {
    fn from(n: i64) -> Self {
        Primitive::Integer(n)
    }
}

impl From<f64> for Primitive {
    fn from(n: f64) -> Self {
        Primitive::Float(n)
    }
}

impl From<String> for Primitive {
    fn from(n: String) -> Self {
        Primitive::Str(n)
    }
}

impl From<bool> for Primitive {
    fn from(n: bool) -> Self {
        Primitive::Boolean(n)
    }
}

impl Primitive {
    fn negate(&self) -> Result<Self, Error> {
        match self {
            Boolean(i) => Ok(Boolean(!*i)),
            left => Self::error(left, None, OperatorError::Negate)
        }
    }

    fn and(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left && *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Add)?
        };
        Ok(res)
    }

    fn or(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Boolean(left), Boolean(right)) => (*left || *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Or)?
        };
        Ok(res)
    }

    fn eq(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left == *right).into(),
            (Boolean(left), Boolean(right)) => (*left == *right).into(),
            (Float(left), Float(right)) => (*left == *right).into(),
            (Float(left), Integer(right)) => (*left == (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) == *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::Equal)?
        };
        Ok(res)
    }

    fn not_eq(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left != *right).into(),
            (Boolean(left), Boolean(right)) => (*left != *right).into(),
            (Float(left), Float(right)) => (*left != *right).into(),
            (Float(left), Integer(right)) => (*left != (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) != *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::NotEqual)?
        };
        Ok(res)
    }

    fn less_than(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left < *right).into(),
            (Float(left), Float(right)) => (*left < *right).into(),
            (Float(left), Integer(right)) => (*left < (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) < *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThan)?
        };
        Ok(res)
    }

    fn less_than_equal(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left <= *right).into(),
            (Float(left), Float(right)) => (*left <= *right).into(),
            (Float(left), Integer(right)) => (*left <= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) <= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::LessThanEqual)?
        };
        Ok(res)
    }

    fn greater_than(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left > *right).into(),
            (Float(left), Float(right)) => (*left > *right).into(),
            (Float(left), Integer(right)) => (*left > (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) > *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThan)?
        };
        Ok(res)
    }

    fn greater_than_equal(&self, other: &Self) -> Result<Self, Error> {
        let res = match (self, other) {
            (Integer(left), Integer(right)) => (*left >= *right).into(),
            (Float(left), Float(right)) => (*left >= *right).into(),
            (Float(left), Integer(right)) => (*left >= (*right as f64)).into(),
            (Integer(left), Float(right)) => ((*left as f64) >= *right).into(),
            (left, right) => Self::error(left, Some(right), OperatorError::GreaterThanEqual)?
        };
        Ok(res)
    }

    fn error<T>(left: &Self, right: Option<&Self>, op: OperatorError) -> Result<T, Error> {
        Err(Error::InvalidOperation(op, left.clone(), right.cloned()))
    }
}

mod primitive_tests {
    use itertools::assert_equal;

    use super::*;

    #[test]
    fn primitive_negate() {
        let truth = Primitive::Boolean(true);
        assert_eq!(Primitive::Boolean(false), truth.negate().unwrap())
    }

    #[test]
    fn primitive_and() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.and(&falsy).unwrap())
    }

    #[test]
    fn primitive_or() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.or(&falsy).unwrap())
    }

    #[test]
    fn primitive_eq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(false), truth.eq(&falsy).unwrap())
    }

    #[test]
    fn primitive_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(true), a.eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_boolean() {
        let truth = Primitive::Boolean(true);
        let falsy = Primitive::Boolean(false);

        assert_eq!(Primitive::Boolean(true), truth.not_eq(&falsy).unwrap())
    }

    #[test]
    fn primitive_noteq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_noteq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);

        assert_eq!(Primitive::Boolean(false), a.not_eq(&b).unwrap())
    }

    #[test]
    fn primitive_less_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.less_than(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_less_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.less_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(false), a.greater_than(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_int() {
        let a = Primitive::Integer(1);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_float() {
        let a = Primitive::Float(1.2);
        let b = Primitive::Float(1.2);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_int_float() {
        let a = Primitive::Integer(1);
        let b = Primitive::Float(1.0);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }

    #[test]
    fn primitive_greater_eq_float_int() {
        let a = Primitive::Float(1.0);
        let b = Primitive::Integer(1);
        assert_eq!(Primitive::Boolean(true), a.greater_than_equal(&b).unwrap())
    }
}