use crate::token::Token;

#[derive(Clone)]
pub struct Tokens {
    pub data: Vec<Token>,
    pos: usize,
}

impl Tokens {
    pub(crate) fn init(data: Vec<Token>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn current(&self) -> Option<Token> {
        self.data.get(self.pos()).cloned().map(|s| s)
    }

    pub fn peek_ahead(&self, pos: usize) -> Option<Token> {
        self.data.get(self.pos + pos).cloned().map(|s| s)
    }

    pub fn peek(&self) -> Option<Token> {
        self.peek_ahead(1)
    }

    pub fn advance(&mut self) {
        self.pos += 1
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.pos += 1;
        self.current()
    }
}