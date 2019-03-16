use crate::ast::{Comparison, Expression, Program, BooleanOperation, UnaryOperation, Operator};
use crate::ast::Statement;
use crate::error::Error;
use crate::error::Error::OtherError;
use crate::object::Object;

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
            println!("{:?}", statement);
            obj = self.visit_statement(statement)?;
        }

        Ok(obj)
    }

    fn visit_statement(&self, statement: Statement) -> Result<Object> {
        match statement {
            Statement::Expr { expression } => self.visit_expression(expression),
            _ => Err(Error::OtherError("Deu ruim".into()))
        }
    }

    /// Should return a `object` instead of `String`.
    fn visit_expression(&self, expression: Expression) -> Result<Object> {
        match expression {
//            Expression::IfExpression { test, body, orelse } => {
//                println!("test: {:?}", test);
//                println!("body: {:?}", body);
//                println!("orelse: {:?}", orelse);
//            }
//            Expression::Call { function, args, keywords } => {
//                println!("function: {:?}", function);
//                println!("args: {:?}", args);
//                println!("keywords: {:?}", keywords);
//            }
            Expression::BinOp { a, op, b } => {
                let a_obj = self.visit_expression(*a)?;
                let b_obj = self.visit_expression(*b)?;
                match op {
                    Operator::Add => a_obj.add(&b_obj),
                    Operator::Sub => a_obj.subtract(&b_obj),
                    Operator::Mul => a_obj.multiply(&b_obj),
                    Operator::Div => a_obj.real_divide(&b_obj),
                    Operator::Mod => a_obj.module(&b_obj),
                    _ => Err(Error::OtherError("Not Implemented yet!".into()))
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
                    _ => Err(Error::OtherError("Not Implemented yet".into()))
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
            Expression::Str { value } => {
                Ok(value.into())
            }
            Expression::Num { value } => {
                Ok(value.into())
            }
            Expression::True => Ok(true.into()),
            Expression::False => Ok(false.into()),
//            Expression::Identifier { name } => {
//                println!("identifier: {:?}", name);
//            }
            _ => Err(Error::OtherError("Deu ruim".into()))
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
    use crate::primitive::Primitive::{Boolean, Integer, Float};

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