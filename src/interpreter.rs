use crate::ast::{Comparison, Expression, Program, BooleanOperation};
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
//            Expression::BinOp { a, op, b } => {
//                println!("value a: {:?}", a);
//                println!("op: {:?}", op);
//                println!("value b: {:?}", b);
//            }
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
//            Expression::UnOp { op, a } => {
//                println!("value a: {:?}", a);
//                println!("unary op: {:?}", op);
//            }
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