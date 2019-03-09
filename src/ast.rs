#[derive(Debug, PartialEq)]
pub enum UnaryOperation {
    Minus,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum BooleanOperation {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug, PartialEq)]
pub enum Comparisson {
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
        op: Comparisson,
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