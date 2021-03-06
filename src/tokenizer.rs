use itertools::{multipeek, MultiPeek};

use crate::token::Token;
use std::str;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    position: usize,
    current_lexeme: String,
    source: MultiPeek<str::Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn init(source: &'a str) -> Self {
        Tokenizer {
            position: 0,
            current_lexeme: "".into(),
            source: multipeek(source.chars()),
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        let current = self.source.next();
        if let Some(c) = current {
            self.current_lexeme.push(c);
        }

        current
    }

    pub fn advance_if_match(&mut self, expected: char) -> bool {
        if self.peek_check(&|c| c == expected) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn advance_while(&mut self, condition: &dyn Fn(char) -> bool) {
        while self.peek_check(condition) {
            self.advance();
        }
    }

    fn peek_check(&mut self, check: &dyn Fn(char) -> bool) -> bool {
        self.source.reset_peek();

        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }

    fn peek_check_two(
        &mut self,
        cond1: &dyn Fn(char) -> bool,
        cond2: &dyn Fn(char) -> bool,
    ) -> bool {
        self.source.reset_peek();

        match self.source.peek() {
            Some(&c) => match self.source.peek() {
                Some(&c2) => cond1(c) && cond2(c2),
                None => false,
            },
            None => false,
        }
    }

    pub fn scan_next(&mut self) -> Option<Token> {
        self.current_lexeme.clear();

        let curr_char = match self.advance() {
            Some(c) => c,
            None => return None,
        };

        match curr_char {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '=' => {
                if self.advance_if_match('=') {
                    Some(Token::EqualEqual)
                } else {
                    Some(Token::Equal)
                }
            }
            '%' => Some(Token::Percent),
            '#' => {
                self.advance_while(&|c| c != '\n');
                Some(Token::Comment)
            }
            '!' => {
                if self.advance_if_match('=') {
                    Some(Token::BangEqual)
                } else {
                    Some(Token::Bang)
                }
            }
            ':' => Some(Token::Colon),
            ',' => Some(Token::Comma),
            '.' => Some(Token::Dot),
            '>' => {
                if self.advance_if_match('=') {
                    Some(Token::GreaterThan)
                } else {
                    Some(Token::Greater)
                }
            }
            '<' => {
                if self.advance_if_match('=') {
                    Some(Token::LessThan)
                } else {
                    Some(Token::Less)
                }
            }
            '(' => Some(Token::ParentOpen),
            ')' => Some(Token::ParentClose),
            '[' => Some(Token::BraceOpen),
            ']' => Some(Token::BraceClose),
            '{' => Some(Token::BracketOpen),
            '}' => Some(Token::BracketClose),
            '"' => self.consume_string(),
            c if c.is_numeric() => self.consume_numbers(),
            c if c.is_whitespace() => Some(Token::WhiteSpace),
            c if c.is_alphabetic() => self.consumer_identifier(),
            _ => Some(Token::EOF),
        }
    }

    fn consume_numbers(&mut self) -> Option<Token> {
        let is_digit = |c: char| c.is_numeric();
        self.advance_while(&is_digit);

        if self.peek_check_two(&|c| c == '.', &is_digit) {
            self.advance();
            self.advance_while(&is_digit)
        }

        if self.current_lexeme.contains(".") {
            let value = self
                .current_lexeme
                .parse::<f64>()
                .expect("expected 'real' type");

            Some(Token::Real(value))
        } else {
            let value = self
                .current_lexeme
                .parse::<i64>()
                .expect("expected 'inteiro' type");

            Some(Token::Inteiro(value))
        }
    }

    fn consume_string(&mut self) -> Option<Token> {
        self.advance_while(&|c| c != '"' && c != '\n');
        let literal: String = self.current_lexeme.chars().skip(1).collect();
        // TODO: Return error!
        if !self.advance_if_match('"') {
            return None;
        }
        Some(Token::Texto(literal))
    }

    fn consumer_identifier(&mut self) -> Option<Token> {
        let is_alpha = |c: char| c.is_alphanumeric();
        self.advance_while(&is_alpha);

        match self.current_lexeme.as_ref() {
            "Verdadeiro" => Some(Token::Logico(true)),
            "Falso" => Some(Token::Logico(false)),
            "Vazio" => Some(Token::Vazio),
            "classe" => Some(Token::Classe),
            "fun" => Some(Token::Fun),
            "lista" => Some(Token::Lista),
            "dicionario" => Some(Token::Dicionario),
            "tupla" => Some(Token::Tupla),
            "conjunto" => Some(Token::Conjunto),
            "imprima" => Some(Token::Imprima),
            "entrada" => Some(Token::Entrada),
            "se" => Some(Token::Se),
            "senao" => Some(Token::SeNao),
            "ouentaose" => Some(Token::OuEntaoSe),
            "e" => Some(Token::E),
            "ou" => Some(Token::Ou),
            "nao" => Some(Token::Nao),
            "é" => Some(Token::Is),
            "remova" => Some(Token::Remova),
            "interrompa" => Some(Token::Interrompa),
            "retorne" => Some(Token::Retorne),
            "continue" => Some(Token::Continue),
            "paracada" => Some(Token::ParaCada),
            "enquanto" => Some(Token::Enquanto),
            "verifique" => Some(Token::Verifique),
            "passe" => Some(Token::Passe),
            "tente" => Some(Token::Tente),
            "exceto" => Some(Token::Exceto),
            "provoque" => Some(Token::Provoque),
            "global" => Some(Token::Global),
            "em" => Some(Token::Em),
            identifier => Some(Token::Identifier(identifier.into())),
        }
    }
}

pub struct TokenizerIterator<'a> {
    tokenizer: Tokenizer<'a>,
}

impl<'a> Iterator for TokenizerIterator<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.tokenizer.scan_next()
    }
}

fn tokenizer_into_iterator<'a>(source: &'a str) -> impl Iterator<Item = Token> + 'a {
    TokenizerIterator {
        tokenizer: Tokenizer::init(source),
    }
}

pub(crate) fn scan(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for token in tokenizer_into_iterator(source) {
        match token {
            Token::WhiteSpace | Token::Comment => {}
            _ => tokens.push(token),
        }
    }

    tokens
}

mod tests {
    use super::*;

    #[test]
    fn test_advance_sum_numbers_success() {
        let mut tokenizer = Tokenizer::init("1 + 1");

        assert_eq!(Some('1'), tokenizer.advance());
        assert_eq!(Some(' '), tokenizer.advance());
        assert_eq!(Some('+'), tokenizer.advance());
        assert_eq!(Some(' '), tokenizer.advance());
        assert_eq!(Some('1'), tokenizer.advance());
    }

    #[test]
    fn test_scan_next_symbols_success() {
        let mut tokenizer = Tokenizer::init(":+-*/()%.,![]{}>< =");

        assert_eq!(Some(Token::Colon), tokenizer.scan_next());
        assert_eq!(Some(Token::Plus), tokenizer.scan_next());
        assert_eq!(Some(Token::Minus), tokenizer.scan_next());
        assert_eq!(Some(Token::Star), tokenizer.scan_next());
        assert_eq!(Some(Token::Slash), tokenizer.scan_next());
        assert_eq!(Some(Token::ParentOpen), tokenizer.scan_next());
        assert_eq!(Some(Token::ParentClose), tokenizer.scan_next());
        assert_eq!(Some(Token::Percent), tokenizer.scan_next());
        assert_eq!(Some(Token::Dot), tokenizer.scan_next());
        assert_eq!(Some(Token::Comma), tokenizer.scan_next());
        assert_eq!(Some(Token::Bang), tokenizer.scan_next());
        assert_eq!(Some(Token::BraceOpen), tokenizer.scan_next());
        assert_eq!(Some(Token::BraceClose), tokenizer.scan_next());
        assert_eq!(Some(Token::BracketOpen), tokenizer.scan_next());
        assert_eq!(Some(Token::BracketClose), tokenizer.scan_next());
        assert_eq!(Some(Token::Greater), tokenizer.scan_next());
        assert_eq!(Some(Token::Less), tokenizer.scan_next());
        assert_eq!(Some(Token::WhiteSpace), tokenizer.scan_next());
        assert_eq!(Some(Token::Equal), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_comment_cases() {
        let mut tokenizer = Tokenizer::init("# isto é um comentario");

        assert_eq!(Some(Token::Comment), tokenizer.scan_next());
        assert_eq!(None, tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_composite_symbols() {
        let mut tokenizer = Tokenizer::init("!=>===<=");

        assert_eq!(Some(Token::BangEqual), tokenizer.scan_next());
        assert_eq!(Some(Token::GreaterThan), tokenizer.scan_next());
        assert_eq!(Some(Token::EqualEqual), tokenizer.scan_next());
        assert_eq!(Some(Token::LessThan), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_strings() {
        let mut tokenizer = Tokenizer::init(r#""coisei""#);
        assert_eq!(Some(Token::Texto("coisei".into())), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_integers() {
        let mut tokenizer = Tokenizer::init("123");
        assert_eq!(Some(Token::Inteiro(123)), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_float() {
        let mut tokenizer = Tokenizer::init("199.00");
        assert_eq!(Some(Token::Real(199.00)), tokenizer.scan_next());
        //        FIXME: I can have .199 float values
        //        let mut tokenizer = Tokenizer::init(".199");
        //        assert_eq!(Some(Token::Real(0.199)), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_spaces() {
        let source = r#":
        "#;
        let mut tokenizer = Tokenizer::init(source);
        assert_eq!(Some(Token::Colon), tokenizer.scan_next());
        assert_eq!(Some(Token::WhiteSpace), tokenizer.scan_next());
        assert_eq!(Some(Token::WhiteSpace), tokenizer.scan_next());
        assert_eq!(Some(Token::WhiteSpace), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_keywords_type_bool_identifiers() {
        let mut tokenizer = Tokenizer::init("Verdadeiro");
        assert_eq!(Some(Token::Logico(true)), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("Falso");
        assert_eq!(Some(Token::Logico(false)), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_class_def_identifier() {
        let mut tokenizer = Tokenizer::init("fun");
        assert_eq!(Some(Token::Fun), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("classe");
        assert_eq!(Some(Token::Classe), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_type_identifier() {
        let mut tokenizer = Tokenizer::init("lista");
        assert_eq!(Some(Token::Lista), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("conjunto");
        assert_eq!(Some(Token::Conjunto), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("tupla");
        assert_eq!(Some(Token::Tupla), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("dicionario");
        assert_eq!(Some(Token::Dicionario), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_input_print_identifier() {
        let mut tokenizer = Tokenizer::init("imprima");
        assert_eq!(Some(Token::Imprima), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("entrada");
        assert_eq!(Some(Token::Entrada), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_conditional_identifiers() {
        let mut tokenizer = Tokenizer::init("se");
        assert_eq!(Some(Token::Se), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("senao");
        assert_eq!(Some(Token::SeNao), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("ouentaose");
        assert_eq!(Some(Token::OuEntaoSe), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_keywords_logic_operators_identifier() {
        let mut tokenizer = Tokenizer::init("e");
        assert_eq!(Some(Token::E), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("ou");
        assert_eq!(Some(Token::Ou), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("nao");
        assert_eq!(Some(Token::Nao), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_keywords_break_continue_and_return_identifier() {
        let mut tokenizer = Tokenizer::init("interrompa");
        assert_eq!(Some(Token::Interrompa), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("continue");
        assert_eq!(Some(Token::Continue), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("retorne");
        assert_eq!(Some(Token::Retorne), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_del_is_and_in_identifier() {
        let mut tokenizer = Tokenizer::init("remova");
        assert_eq!(Some(Token::Remova), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("é");
        assert_eq!(Some(Token::Is), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("em");
        assert_eq!(Some(Token::Em), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_for_while_and_continue_identifier() {
        let mut tokenizer = Tokenizer::init("paracada");
        assert_eq!(Some(Token::ParaCada), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("continue");
        assert_eq!(Some(Token::Continue), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("enquanto");
        assert_eq!(Some(Token::Enquanto), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_pass_assert_identifier() {
        let mut tokenizer = Tokenizer::init("passe");
        assert_eq!(Some(Token::Passe), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("verifique");
        assert_eq!(Some(Token::Verifique), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_keywords_try_except_raise_identifier() {
        let mut tokenizer = Tokenizer::init("tente");
        assert_eq!(Some(Token::Tente), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("exceto");
        assert_eq!(Some(Token::Exceto), tokenizer.scan_next());

        let mut tokenizer = Tokenizer::init("provoque");
        assert_eq!(Some(Token::Provoque), tokenizer.scan_next());
    }

    #[test]
    fn test_scan_next_keywords_global_identifier() {
        let mut tokenizer = Tokenizer::init("global");
        assert_eq!(Some(Token::Global), tokenizer.scan_next())
    }

    #[test]
    fn test_scan_next_variable_identifier() {
        let mut tokenizer = Tokenizer::init("num = 0");
        assert_eq!(Some(Token::Identifier("num".into())), tokenizer.scan_next())
    }
}

mod scan {
    use super::*;

    #[test]
    fn test_scan() {
        let source = "se Verdadeiro:";
        let tokens = scan(source);

        assert_eq!(3, tokens.len());
        assert_eq!(Token::Se, tokens[0]);
        assert_eq!(Token::Logico(true), tokens[1]);
        assert_eq!(Token::Colon, tokens[2]);
    }

    #[test]
    fn test_scan_code_with_more_lines() {
        let source = r#"se Verdadeiro:
            imprima("oi")
        "#;
        let tokens = scan(source);
        assert_eq!(7, tokens.len());
        assert_eq!(Token::Se, tokens[0]);
        assert_eq!(Token::Logico(true), tokens[1]);
        assert_eq!(Token::Colon, tokens[2]);
        assert_eq!(Token::Imprima, tokens[3]);
        assert_eq!(Token::ParentOpen, tokens[4]);
        assert_eq!(Token::Texto("oi".into()), tokens[5]);
        assert_eq!(Token::ParentClose, tokens[6]);
    }

    #[test]
    fn test_scan_code_with_more_lines_should_ignore_comments() {
        let source = r#"
        se Verdadeiro: # Devo ignorar este comentario
            imprima("oi")
        "#;
        let tokens = scan(source);
        assert_eq!(7, tokens.len());
        assert_eq!(Token::Se, tokens[0]);
        assert_eq!(Token::Logico(true), tokens[1]);
        assert_eq!(Token::Colon, tokens[2]);
        assert_eq!(Token::Imprima, tokens[3]);
        assert_eq!(Token::ParentOpen, tokens[4]);
        assert_eq!(Token::Texto("oi".into()), tokens[5]);
        assert_eq!(Token::ParentClose, tokens[6]);
    }
}
