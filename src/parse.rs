use crate::ast;
use crate::tokenizer::scan;
use crate::token::Token;
use crate::tokens::Tokens;
use crate::ast::{Statement, Expression};
use crate::token::Token::ParentClose;

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
                    eprintln!("got error: ParseError {:?}", error);
                    errors.push(error);
                    break;
                }
            }
        }
        Some(ast::Program { statements: statements })
    }

    fn statements(&mut self) -> Option<Result<Statement, ParseError>> {
        match self.current() {
            // fun
            // classe
            // identifier
            Some(Token::Identifier(value)) => match self.peek() {
                Some(Token::Equal) => {
                    let _ = self.advance();
                    if let Some(assign) = self.parse_assignment() {
                        Some(Ok(assign))
                    } else {
                        Some(Err(ParseError::Message("could not get assignment".into())))
                    }
                }
                _ => None
            }
            // imprima
            Some(Token::Imprima) => {
                let _ = self.advance();
                if let Some(expression) = self.parser_print() {
                    Some(Ok(ast::Statement::Expr { expression }))
                } else {
                    Some(Err(ParseError::Message("could not get statements".into())))
                }
            }
            _ => None
        }
    }

    fn expression(self) -> ast::Expression {
        unimplemented!()
    }

    fn call(&self) {
        unimplemented!()
    }

    fn parser_print(&mut self) -> Option<ast::Expression> {
        if !self.consume(&Token::ParentOpen) {
            return None;
        }

        let args = match self.current() {
            Some(Token::Texto(msg)) => {
                let _ = self.advance();
                vec![ast::Expression::Str { value: msg }]
            }
            _ => vec![ast::Expression::Str { value: "".into() }]
        };
        if !self.consume(&Token::ParentClose) {
            return None;
        }

        Some(ast::Expression::Call {
            function: Box::new(ast::Expression::Identifier { name: String::from("imprima") }),
            args: args,
            keywords: vec![],
        })
    }

    fn parse_assignment(&mut self) -> Option<ast::Statement> {
        // =
        // value (expression)
        unimplemented!()
    }

    fn identifier(&mut self) -> Option<ast::Expression> {
        unimplemented!()
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
    use super::ast;
    use super::parse_program;
    use crate::ast::Number;

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
    fn test_assign_variable() {
        let parse_ast = parse_program(r#"numero = 10"#);
        assert_eq!(parse_ast, Some(ast::Program {
            statements: vec![ast::Statement::Assign {
                targets: vec![ast::Expression::Identifier { name: "numero".into() }],
                value: ast::Expression::Num { value: Number::Integer { value: 10 } },
            }]
        }))
    }
}