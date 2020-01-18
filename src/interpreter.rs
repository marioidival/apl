use crate::ast::Statement;
use crate::ast::{BooleanOperation, Comparison, Expression, Operator, Program, UnaryOperation};
use crate::error::Error;
use crate::error::Error::OtherError;
use crate::object::Object;
use crate::primitive::Primitive;

type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
struct Interpreter;

impl Interpreter {
    pub fn init() -> Self {
        Self
    }

    pub fn eval(&self, program: Program) -> Result<Object> {
        Ok(self.visit_program(program)?)
    }

    fn visit_program(&self, program: Program) -> Result<Object> {
        let Program { statements, .. } = program;

        let mut obj = Object::Unit;
        for statement in statements {
            obj = self.visit_statement(statement)?;
        }

        Ok(obj)
    }

    fn visit_statement(&self, statement: Statement) -> Result<Object> {
        match statement {
            Statement::Expr { expression } => self.visit_expression(expression),
            _ => Err(Error::OtherError("statement not implemented yet".into())),
        }
    }

    fn visit_expression(&self, expression: Expression) -> Result<Object> {
        match expression {
            Expression::IfExpression { test, body, orelse } => {
                match self.visit_expression(*test)? {
                    Object::Primitive(Primitive::Boolean(true)) => self.visit_expression(*body),
                    Object::Primitive(Primitive::Boolean(false)) => self.visit_expression(*orelse),
                    _ => Err(OtherError("should be true or false".into())),
                }
            }
            //            Expression::Call { function, args, keywords } => {}
            Expression::BinOp { a, op, b } => {
                let a_obj = self.visit_expression(*a)?;
                let b_obj = self.visit_expression(*b)?;
                match op {
                    Operator::Add => a_obj.add(&b_obj),
                    Operator::Sub => a_obj.subtract(&b_obj),
                    Operator::Mul => a_obj.multiply(&b_obj),
                    Operator::Div => a_obj.real_divide(&b_obj),
                    Operator::Mod => a_obj.module(&b_obj),
                }
            }
            Expression::Compare { a, op, b } => {
                let a_obj = self.visit_expression(*a)?;
                let b_obj = self.visit_expression(*b)?;
                match op {
                    Comparison::Equal => a_obj.equal(&b_obj),
                    Comparison::NotEqual => a_obj.not_equal(&b_obj),
                    Comparison::Greater => a_obj.greater_than(&b_obj),
                    Comparison::Less => a_obj.less_than(&b_obj),
                    Comparison::GreaterThan => a_obj.greater_than_equal(&b_obj),
                    Comparison::LessThan => a_obj.less_than_equal(&b_obj),
                    Comparison::Is => a_obj.is(&b_obj),
                    _ => Err(Error::OtherError("comparison not implemented yet".into())),
                }
            }
            Expression::BoolOp { a, op, b } => {
                let a_obj = self.visit_expression(*a)?;
                let b_obj = self.visit_expression(*b)?;
                match op {
                    BooleanOperation::And => a_obj.and(&b_obj),
                    BooleanOperation::Or => a_obj.or(&b_obj),
                }
            }
            Expression::UnOp { op, a } => {
                let a_obj = self.visit_expression(*a)?;
                match op {
                    UnaryOperation::Not => a_obj.negate(),
                    UnaryOperation::Minus => a_obj.unary_minus(),
                    UnaryOperation::Plus => a_obj.unary_plus(),
                }
            }
            Expression::Str { value } => Ok(value.into()),
            Expression::Num { value } => Ok(value.into()),
            Expression::True => Ok(true.into()),
            Expression::False => Ok(false.into()),
            //            Expression::Identifier { name } => {
            //                println!("identifier: {:?}", name);
            //            }
            _ => Err(Error::OtherError("expression not implemented yet".into())),
        }
    }
}

#[cfg(test)]
mod comparison {
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::parse::parse_program;
    use crate::primitive::Primitive::Boolean;

    #[test]
    fn equal() {
        let mut parser_ast = parse_program(r#"1 == 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(false)), result.unwrap())
    }

    #[test]
    fn not_equal() {
        let mut parser_ast = parse_program(r#"1 != 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }

    #[test]
    fn greater() {
        let mut parser_ast = parse_program(r#"1 > 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }

    #[test]
    fn less() {
        let mut parser_ast = parse_program(r#"1 < 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(false)), result.unwrap())
    }

    #[test]
    fn greater_equal() {
        let mut parser_ast = parse_program(r#"1 >= 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }

    #[test]
    fn less_equal() {
        let mut parser_ast = parse_program(r#"1 <= 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(false)), result.unwrap())
    }

    #[test]
    fn is() {
        let mut parser_ast = parse_program(r#"1 é 0"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }
}

#[cfg(test)]
mod bool_operation {
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::parse::parse_program;
    use crate::primitive::Primitive::Boolean;

    #[test]
    fn and() {
        let mut parser_ast = parse_program(r#"Verdadeiro e Verdadeiro"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }

    #[test]
    fn or() {
        let mut parser_ast = parse_program(r#"Verdadeiro e Verdadeiro"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }
}

#[cfg(test)]
mod unary_operation {
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::parse::parse_program;
    use crate::primitive::Primitive::{Boolean, Integer};

    #[test]
    fn not() {
        let mut parser_ast = parse_program(r#"nao Falso"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap())
    }

    #[test]
    fn minus() {
        let mut parser_ast = parse_program(r#"-1"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(-1)), result.unwrap())
    }

    #[test]
    fn plus() {
        // FIXME: Add capacity of work with (), e.g: +(-1)
        let mut parser_ast = parse_program(r#"1"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(1)), result.unwrap())
    }
}

#[cfg(test)]
mod binary_operation {
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::parse::parse_program;
    use crate::primitive::Primitive::{Boolean, Float, Integer};

    #[test]
    fn add() {
        let mut parser_ast = parse_program(r#"9 + 3"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(12)), result.unwrap())
    }

    #[test]
    fn sub() {
        let mut parser_ast = parse_program(r#"9 - 3"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(6)), result.unwrap())
    }

    #[test]
    fn mul() {
        let mut parser_ast = parse_program(r#"9 * 3"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(27)), result.unwrap())
    }

    #[test]
    fn div() {
        let mut parser_ast = parse_program(r#"9 / 3"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Float(3.0)), result.unwrap())
    }

    #[test]
    fn module() {
        let mut parser_ast = parse_program(r#"9 % 3"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(0)), result.unwrap())
    }
}

#[cfg(test)]
mod ifexpression {
    use crate::interpreter::Interpreter;
    use crate::object::Object;
    use crate::parse::parse_program;
    use crate::primitive::Primitive::{Boolean, Integer};

    #[test]
    fn if_condition_true() {
        let parser_ast = parse_program(r#"se 1 > 0: Verdadeiro"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Boolean(true)), result.unwrap());
    }

    #[test]
    fn if_condition_false() {
        let parser_ast = parse_program(r#"se 1 < 0: 1 + 1 senao: 1 - 1"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(0)), result.unwrap());
    }

    #[test]
    fn if_multi_condition() {
        let parser_ast = parse_program(r#"se 1 < 0: 1 + 1 senao: se Verdadeiro: 5 - 1"#);
        let interpreter = Interpreter::init();
        let result = interpreter.eval(parser_ast.unwrap());
        assert_eq!(Object::Primitive(Integer(4)), result.unwrap());
    }
}
