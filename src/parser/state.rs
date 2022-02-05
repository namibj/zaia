use std::ops::Range;

use logos::Logos;

use super::token::Token;
use crate::T;
use crate::intern::Interner;
use crate::engine::gc::Handle;
use crate::engine::value::RefValue;

pub struct State<'source> {
    interner: &'source mut Interner,
    tokens: Vec<(Token, Range<usize>)>,
    cursor: usize,
    source: &'source str,
    reports: Vec<ariadne::Report>,
}

impl<'source> State<'source> {
    pub fn new(interner: &'source mut Interner, source: &'source str) -> Self {
        let mut tokens = vec![(T![eof], 0..0)];
        tokens.extend(Token::lexer(source).spanned());

        State {
            interner,
            tokens,
            cursor: 0,
            source,
            reports: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Token {
        self.tokens
            .get(self.cursor + 1)
            .map(|(token, _)| *token)
            .unwrap_or(T![eof])
    }

    pub fn eat(&mut self, token: Token) {
        self.bump();
        let span = self.span();
        let found = self
            .tokens
            .get(self.cursor)
            .map(|(token, _)| *token)
            .unwrap_or(T![eof]);

        if found != token {
            panic!("found unexpected token {} at {:?}", found, span);
        }
    }

    pub fn bump(&mut self) {
        self.cursor += 1;
    }

    pub fn span(&self) -> Range<usize> {
        self.tokens[self.cursor].1.clone()
    }

    pub fn slice(&self) -> &'source str {
        let span = self.span();
        &self.source[span.start..span.end]
    }

    pub fn intern<T>(&mut self, item: &T) -> Handle<RefValue>
    where
        T: AsRef<[u8]>,
{
    self.interner.intern(item)
}

    pub fn result(self) -> Vec<ariadne::Report> {
        self.reports
    }
}
