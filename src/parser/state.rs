use std::ops::Range;

use logos::Logos;

use super::token::Token;
use crate::T;

pub struct State<'source> {
    tokens: Vec<(Token, Range<usize>)>,
    cursor: usize,
    source: &'source str,
    reports: Vec<ariadne::Report>,
}

impl<'source> State<'source> {
    pub fn new(source: &'source str) -> Self {
        let tokens = Token::lexer(source).spanned().collect();

        State {
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
        let found = self.next();

        if found != token {
            panic!("found unexpected token {}", found);
        }
    }

    pub fn next(&mut self) -> Token {
        let position = self.cursor;
        self.cursor += 1;

        self.tokens
            .get(position)
            .map(|(token, _)| *token)
            .unwrap_or(T![eof])
    }

    pub fn current(&self) -> Token {
        self.tokens[self.cursor].0
    }

    pub fn span(&self) -> Range<usize> {
        self.tokens[self.cursor].1.clone()
    }

    pub fn slice(&self) -> &str {
        let span = self.span();
        &self.source[span.start..span.end]
    }

    pub fn report(&mut self, report: ariadne::Report) {
        self.reports.push(report);
    }

    pub fn result(self) -> Vec<ariadne::Report> {
        self.reports
    }
}
