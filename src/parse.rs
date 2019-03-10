use crate::ast;
use crate::ast::{BooleanOperation, Comparison, Expression, Number, Operator, Statement, UnaryOperation};
use crate::token::Token;
use crate::tokenizer::scan;
use crate::tokens::Tokens;

#[derive(Debug)]
pub enum ParseError {
    Message(String)
}

/// It takes the list of tokens as input and create an AST as output.
pub struct Parser {
    tokens: Tokens,
}

impl Parser {
    pub fn init(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: Tokens::init(tokens),
        }
    }

    fn parser(&mut self) -> Option<ast::Program> {
        let mut statements: Vec<Statement> = Vec::new();
        let mut errors: Vec<ParseError> = Vec::new();
        while let Some(result) = self.statements() {
            match result {
                Ok(statement) => {
                    statements.push(statement);
                }
                Err(error) => {
                    errors.push(error);
                    break;
                }
            }
        }
        Some(ast::Program { statements: statements })
    }

    fn statements(&mut self) -> Option<Result<Statement, ParseError>> {
        match self.current() {
            Some(Token::Interrompa) => Some(Ok(ast::Statement::Break)),
            Some(Token::Passe) => Some(Ok(ast::Statement::Pass)),
            Some(Token::Retorne) => {
                if let Some(expr) = self.expression() {
                    Some(Ok(ast::Statement::Return { value: Some(vec![expr]) }))
                } else {
                    Some(Ok(ast::Statement::Return { value: Some(vec![]) }))
                }
            }
            Some(Token::Verifique) => {
                if let Some(expr) = self.expression() {
                    Some(Ok(ast::Statement::Assert {
                        test: expr,
                        msg: None,
                    }))
                } else {
                    None
                }
            }
            Some(Token::Identifier(_)) => match self.peek() {
                Some(Token::Equal) => {
                    if let Some(assign) = self.parse_assignment() {
                        Some(Ok(assign))
                    } else {
                        Some(Err(ParseError::Message("could not get assignment".into())))
                    }
                }
                _ => None
            }
            Some(Token::Imprima) => {
                if let Some(expression) = self.parse_print() {
                    Some(Ok(ast::Statement::Expr { expression }))
                } else {
                    Some(Err(ParseError::Message("could not get statements".into())))
                }
            }
            _ => {
                if let Some(expression) = self.expression() {
                    Some(Ok(ast::Statement::Expr { expression }))
                } else {
                    None
                }
            }
        }
    }

    fn parse_assignment(&mut self) -> Option<ast::Statement> {
        let identifier = match self.identifier() {
            Ok(identifier) => identifier,
            Err(o) => {
                return None;
            }
        };

        if !self.consume(&Token::Equal) {
            return None;
        }

        Some(ast::Statement::Assign {
            targets: vec![identifier],
            value: self.expression().unwrap(),
        })
    }

    fn expression(&mut self) -> Option<ast::Expression> {
        let mut a = self.factor();
        while let Some(token) = self.current() {
            match token {
                Token::Less | Token::LessThan | Token::Greater | Token::GreaterThan => {
                    self.advance();
                    a = Some(ast::Expression::Compare {
                        a: Box::new(a.unwrap()),
                        op: Comparison::from(token),
                        b: Box::new(self.factor().unwrap()),
                    });
                }
                Token::Plus | Token::Minus | Token::Slash | Token::Star => {
                    self.advance();
                    a = Some(ast::Expression::BinOp {
                        a: Box::new(a.unwrap()),
                        op: Operator::from(token),
                        b: Box::new(self.factor().unwrap()),
                    });
                }
                Token::E | Token::Ou => {
                    self.advance();
                    a = Some(ast::Expression::BoolOp {
                        a: Box::new(a.unwrap()),
                        op: BooleanOperation::from(token),
                        b: Box::new(self.factor().unwrap()),
                    })
                }
                Token::Nao | Token::Minus => {
                    self.advance();
                    a = Some(ast::Expression::UnOp {
                        op: UnaryOperation::from(token),
                        a: Box::new(self.factor().unwrap()),
                    })
                }
                _ => {
                    break;
                }
            }
        }
        return a;
    }

    fn factor(&mut self) -> Option<ast::Expression> {
        match self.current() {
            Some(Token::Inteiro(value)) => {
                let _ = self.advance();
                Some(ast::Expression::Num {
                    value: Number::Integer { value }
                })
            }
            Some(Token::Real(value)) => {
                let _ = self.advance();
                Some(ast::Expression::Num {
                    value: Number::Float { value }
                })
            }
            Some(Token::Texto(value)) => {
                let _ = self.advance();
                Some(ast::Expression::Str { value })
            }
            Some(Token::Logico(value)) => {
                let _ = self.advance();
                Some(if value { ast::Expression::True } else { ast::Expression::False })
            }
            _ => None,
        }
    }

    fn parse_print(&mut self) -> Option<ast::Expression> {
        let print_identifier = match self.identifier() {
            Ok(print_identifier) => print_identifier,
            Err(_) => return None,
        };

        if !self.consume(&Token::ParentOpen) {
            return None;
        }

        let args = vec![self.expression().unwrap()];

        if !self.consume(&Token::ParentClose) {
            return None;
        }

        Some(ast::Expression::Call {
            function: Box::new(print_identifier),
            args,
            keywords: vec![],
        })
    }

    fn identifier(&mut self) -> Result<ast::Expression, ParseError> {
        match self.current() {
            Some(Token::Imprima) => {
                let _ = self.advance();
                Ok(ast::Expression::Identifier { name: String::from("imprima") })
            }
            Some(Token::Identifier(name)) => {
                let _ = self.advance();
                Ok(ast::Expression::Identifier { name: String::from(name) })
            }
            _ => Err(ParseError::Message("identifier not found".into())),
        }
    }

    fn consume(&mut self, tok: &Token) -> bool {
        match self.current() {
            Some(ref t) if t == tok => {
                self.advance();
                true
            }
            _ => false
        }
    }

    fn current(&self) -> Option<Token> {
        self.tokens.current()
    }

    fn advance(&mut self) -> Option<Token> {
        self.tokens.advance();
        self.current()
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek()
    }

    fn expected(&self) {
        unimplemented!()
    }
}


pub fn parse_program(source: &str) -> Option<ast::Program> {
    let tokens = scan(source);
    let mut parser = Parser::init(tokens);
    parser.parser()
}

#[cfg(test)]
mod tests {
    use crate::ast::{BooleanOperation, Comparison, Number, Operator, UnaryOperation};

    use super::ast;
    use super::parse_program;

    #[test]
    fn test_parse_empty() {
        let parse_ast = parse_program("\n");

        assert_eq!(parse_ast, Some(ast::Program { statements: vec![] }))
    }

    #[test]
    fn test_hello_world() {
        let parse_ast = parse_program(r#"imprima("Olá mundo!")"#);

        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![ast::Statement::Expr {
                expression: ast::Expression::Call {
                    function: Box::new(ast::Expression::Identifier { name: String::from("imprima") }),
                    args: vec![ast::Expression::Str { value: String::from("Olá mundo!") }],
                    keywords: vec![],
                }
            }]
        }))
    }

    #[test]
    fn test_two_hello_world() {
        let parse_ast = parse_program(r#"
            imprima("Olá mundo!")
            imprima("Olá pequeno gafanhoto!")
        "#);

        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::Call {
                        function: Box::new(ast::Expression::Identifier { name: String::from("imprima") }),
                        args: vec![ast::Expression::Str { value: String::from("Olá mundo!") }],
                        keywords: vec![],
                    }
                },
                ast::Statement::Expr {
                    expression: ast::Expression::Call {
                        function: Box::new(ast::Expression::Identifier { name: String::from("imprima") }),
                        args: vec![ast::Expression::Str { value: String::from("Olá pequeno gafanhoto!") }],
                        keywords: vec![],
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_sum_in_print() {
        let parse_ast = parse_program(r#"imprima(5 + 3 - 2)"#);

        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::Call {
                        function: Box::new(ast::Expression::Identifier { name: String::from("imprima") }),
                        args: vec![ast::Expression::BinOp {
                            a: Box::new(ast::Expression::BinOp {
                                a: Box::new(ast::Expression::Num { value: ast::Number::Integer { value: 5 } }),
                                op: Operator::Add,
                                b: Box::new(ast::Expression::Num { value: ast::Number::Integer { value: 3 } }),
                            }),
                            op: Operator::Sub,
                            b: Box::new(ast::Expression::Num { value: ast::Number::Integer { value: 2 } }),
                        }],
                        keywords: vec![],
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_assign_int_variable() {
        let parse_ast = parse_program(r#"numero = 10"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![ast::Statement::Assign {
                targets: vec![ast::Expression::Identifier { name: "numero".into() }],
                value: ast::Expression::Num { value: Number::Integer { value: 10 } },
            }]
        }))
    }

    #[test]
    fn test_assign_float_variable() {
        let parse_ast = parse_program(r#"real = 10.0"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![ast::Statement::Assign {
                targets: vec![ast::Expression::Identifier { name: "real".into() }],
                value: ast::Expression::Num { value: Number::Float { value: 10.0 } },
            }]
        }))
    }

    #[test]
    fn test_assign_string_variable() {
        let parse_ast = parse_program(r#"nome = "coral""#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![ast::Statement::Assign {
                targets: vec![ast::Expression::Identifier { name: "nome".into() }],
                value: ast::Expression::Str { value: "coral".into() },
            }]
        }))
    }

    #[test]
    fn test_comparision_less() {
        let parse_ast = parse_program(r#"1 < 5"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::Compare {
                        a: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 1 }
                        }),
                        op: Comparison::Less,
                        b: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 5 }
                        }),
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_comparision_less_than() {
        let parse_ast = parse_program(r#"1 <= 5"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::Compare {
                        a: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 1 }
                        }),
                        op: Comparison::LessThan,
                        b: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 5 }
                        }),
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_bin_or_comparision() {
        let parse_ast = parse_program(r#"Verdadeiro ou Falso"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::BoolOp {
                        a: Box::new(ast::Expression::True),
                        op: BooleanOperation::Or,
                        b: Box::new(ast::Expression::False),
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_bin_sum_operation() {
        let parse_ast = parse_program(r#"1 + 5"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![
                ast::Statement::Expr {
                    expression: ast::Expression::BinOp {
                        a: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 1 }
                        }),
                        op: Operator::Add,
                        b: Box::new(ast::Expression::Num {
                            value: Number::Integer { value: 5 }
                        }),
                    }
                }
            ]
        }))
    }

    #[test]
    fn test_not_unary_operation() {
        // FIXME: unary minus need be fixed!
        let parse_ast = parse_program(r#"nao Falso"#);
        assert_eq!(
            parse_ast,
            Some(ast::Program {
                statements: vec![
                    ast::Statement::Expr {
                        expression: ast::Expression::UnOp {
                            op: UnaryOperation::Not,
                            a: Box::new(ast::Expression::False),
                        }
                    }
                ]
            })
        )
    }
}