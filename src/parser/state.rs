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
        State {
            tokens: Token::lexer(source).spanned().collect(),
            cursor: 0,
            source,
            reports: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Token {
        self.tokens.get(self.cursor+1).map(|(token, _)| *token).unwrap_or(T![eof])
    }

    pub fn at(&mut self, token: Token) -> bool {
        self.peek() == token
    }

    pub fn eat(&mut self, token: Token) {
        let found = self.next();

        if found != token {
            let found_name = if token == found {
                token.to_string()
            } else {
                "NONE".to_string()
            };

            let found_message = format!("Expected {} but found {}", token, found_name);
            let span = self.span();

            let report =
                ariadne::Report::build(ariadne::ReportKind::Error, (), span.start)
                    .with_message("Unexpected token")
                    .with_label(
                        ariadne::Label::new(span).with_message(found_message),
                    )
                    .finish();

            self.reports.push(report);
        }
    }

    pub fn next(&mut self) -> Token {
        self.cursor += 1;
        self.tokens.get(self.cursor).map(|(token, _)| *token).unwrap_or(T![eof])
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
