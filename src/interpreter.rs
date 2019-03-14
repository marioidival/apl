use crate::ast::{Expression, Program};
use crate::ast::Statement;
use crate::error::InterpreterError;


#[derive(Debug, PartialEq)]
struct Interpreter;

impl Interpreter {
    pub fn init() -> Self {
        Self
    }

    pub fn eval(&self, program: Program) -> Result<String, InterpreterError> {
        Ok(self.visit_program(program)?)
    }

    fn visit_program(&self, program: Program) -> Result<String, InterpreterError> {
        let Program { statements, .. } = program;

        for statement in statements {
            self.visit_statement(statement);
        }

        Ok("Deu certo".into())
    }

    fn visit_statement(&self, statement: Statement) -> Result<String, InterpreterError> {
        match statement {
            Statement::Expr { expression } => self.visit_expression(expression),
            _ => Err(InterpreterError::Message("Deu ruim statement".into()))
        }
    }

    /// Should return a `object` instead of `String`.
    fn visit_expression(&self, expression: Expression) -> Result<String, InterpreterError> {
        match expression {
            Expression::IfExpression { test, body, orelse } => {
                println!("test: {:?}", test);
                println!("body: {:?}", body);
                println!("orelse: {:?}", orelse);
            }
            Expression::Call { function, args, keywords } => {
                println!("function: {:?}", function);
                println!("args: {:?}", args);
                println!("keywords: {:?}", keywords);
            }
            Expression::BinOp { a, op, b } => {
                println!("value a: {:?}", a);
                println!("op: {:?}", op);
                println!("value b: {:?}", b);
            }
            Expression::Compare { a, op, b } => {
                let a_expr = self.visit_expression(*a);
                let b_expr = self.visit_expression(*b);
                println!("value a: {:?}", a_expr);
                println!("comparision: {:?}", op);
                println!("value b: {:?}", b_expr);
            }
            Expression::BoolOp { a, op, b } => {
                println!("value a: {:?}", a);
                println!("bool op: {:?}", op);
                println!("value b: {:?}", b);
            }
            Expression::UnOp { op, a } => {
                println!("value a: {:?}", a);
                println!("unary op: {:?}", op);
            }
            Expression::Str { value } => {
                println!("string value: {:?}", value);
            }
            Expression::Num { value } => {
                println!("num value: {:?}", value);
            }
            Expression::Identifier { name } => {
                println!("identifier: {:?}", name);
            }
            _ => ()
        }
        Ok("Deu bom".into())
    }
}


mod tests {
    use crate::interpreter::Interpreter;
    use crate::parse::parse_program;

    #[test]
    fn interpreter() {
        let mut parser_ast = parse_program(r#"1 == 0"#);
        let interpreter = Interpreter::init();
        let result = match interpreter.eval(parser_ast.unwrap()) {
            Ok(r) => r,
            Err(_) => "Deu ruim".into(),
        };
        assert_eq!(String::from("Falso"), result)
    }
}
