use crate::token::Token;
use crate::ast::Expression::{Compare, BoolOp};

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