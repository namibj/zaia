use logos::{Logos, Lexer};
use super::token::Token;
use std::ops::{Deref,DerefMut};
use crate::T;

pub struct State<'source> {
    lexer: Lexer<'source, Token>,
    peeked: Token,
}

impl<'source> State<'source> {
    pub fn new(source: &'source str) -> Self {
        State {
            lexer: Token::lexer(source),
            peeked: T![eof],
        }
    }

    pub fn peek(&mut self) -> Token {
        if self.peeked == T![eof] {
            if let Some(token) = self.lexer.next() {
                self.peeked = token;
            } else {
                return T![eof];
            }
        }

        self.peeked
    }

    pub fn at(&mut self, token: Token) -> bool {
        self.peek() == token
    }

    pub fn eat(&mut self, token: Token) {
        if self.next() != Some(token) {
            panic!("expected {:?}", token);
        }
    }
}

impl<'source> Iterator for State<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked != T![eof] {
            Some(self.peeked)
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
