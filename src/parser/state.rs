use logos::{Logos, Lexer};
use super::token::Token;
use std::ops::{Deref,DerefMut};

pub struct State<'source> {
    lexer: Lexer<'source, Token>,
    peeked: Option<Token>,
}

impl<'source> State<'source> {
    pub fn new(source: &'source str) -> Self {
        State {
            lexer: Token::lexer(source),
            peeked: None,
        }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        if self.peeked.is_none() {
            self.peeked = self.lexer.next();
        }

        self.peeked.as_ref()
    }
}

impl<'source> Iterator for State<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_some() {
            self.peeked.take()
        } else {
            self.lexer.next()
        }
    }
}

impl<'source> Deref for State<'source> {
    type Target = Lexer<'source, Token>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'source> DerefMut for State<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}
