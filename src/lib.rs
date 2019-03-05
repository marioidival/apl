extern crate itertools;

use std::str;

use itertools::{multipeek, MultiPeek};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Token {
    // int
    Inteiro(i64),
    // float
    Real(f64),
    // str
    Texto(String),
    // bool
    Logico(bool),
    // tuple
    Tupla,
    // list
    Lista,
    // set
    Conjunto,
    // dict
    Dicionario,
    // def
    Fun,
    // class
    Classe,
    // print
    Imprima,
    // if
    Se,
    // else
    SeNao,
    // Elif?
    // and
    E,
    // or
    Ou,
    // not
    Negue,
    // True
    Verdadeiro,
    // False
    Falso,
    // del
    Remova,
    // in
    Em,
    // assert
    Verifique,
    // break
    Interromper,
    // return
    Retorne,
    // None,
    Vazio,
    // continue
    Continue,
    // for
    ParaCada,
    // while
    Enquanto,
    // global
    Global,
    // try
    Tente,
    // except
    Fudeu,
    // pass
    Passe,

    // identifier == variables
    Identifier(String),

    // symbols
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    Percent,
    // %
    Comment,
    Bang,
    Colon,
    // :
    Comma,
    Dot,
    Greater,
    Less,
    ParentOpen,
    ParentClose,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,

    // composite symbols
    EqualEqual,
    GreaterThan,
    LessThan,
    BangEqual,

    // etc
    WhiteSpace,
    EOF,
}

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

    fn advance_while(&mut self, condition: &Fn(char) -> bool) {
        while self.peek_check(condition) {
            self.advance();
        }
    }

    fn peek_check(&mut self, check: &Fn(char) -> bool) -> bool {
        self.source.reset_peek();

        match self.source.peek() {
            Some(&c) => check(c),
            None => false,
        }
    }

    fn peek_check_two(&mut self, cond1: &Fn(char) -> bool, cond2: &Fn(char) -> bool) -> bool {
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
            '#' => Some(Token::Comment),
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

        Some(Token::Texto(literal))
    }
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
        let mut tokenizer = Tokenizer::init(":+-*/()%.,![]{}><#= ");

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
        assert_eq!(Some(Token::Comment), tokenizer.scan_next());
        assert_eq!(Some(Token::Equal), tokenizer.scan_next());
        assert_eq!(Some(Token::WhiteSpace), tokenizer.scan_next());
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
    fn it_works() {
        assert_eq!(true, true)
    }
}
