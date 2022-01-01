use super::token::Token;
use crate::T;
use logos::{Logos, SpannedIter};
use std::ops::{Deref, DerefMut};

pub struct State<'source> {
    lexer: SpannedIter<'source, Token>,
    peeked: (Token, logos::Span),
    reports: Vec<ariadne::Report>,
}

impl<'source> State<'source> {
    pub fn new(source: &'source str) -> Self {
        State {
            lexer: Token::lexer(source).spanned(),
            peeked: (T![eof], logos::Span::default()),
            reports: Vec::new(),
        }
    }

    pub fn peek(&mut self) -> Token {
        if self.peeked.0 == T![eof] {
            if let Some((token, span)) = self.lexer.next() {
                self.peeked = (token, span);
            } else {
                return T![eof];
            }
        }

        self.peeked.0
    }

    pub fn at(&mut self, token: Token) -> bool {
        self.peek() == token
    }

    pub fn eat(&mut self, token: Token) {
        let found = self.next();

        if found == Some(token) {
            let found_message = if let Some(token) = found {
                token.to_string()
            } else {
                "NONE".to_string()
            };

            let report = 
                ariadne::Report::build(ariadne::ReportKind::Error, (), self.peeked.1.start)
                    .with_message("Unexpected token")
                    .with_label(ariadne::Label::new(self.peeked.1.clone()).with_message(found_message))
                    .finish();
            
            self.reports.push(report);
        }
    }

    pub fn report(&mut self, report: ariadne::Report) {
        self.reports.push(report);
    }

    pub fn result(self) -> Vec<ariadne::Report> {
        self.reports
    }
}

impl<'source> Iterator for State<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.0 != T![eof] {
            let token = self.peeked.0;
            self.peeked.0 = T![eof];
            Some(token)
        } else {
            self.lexer.next().map(|(token, _)| token)
        }
    }
}

impl<'source> Deref for State<'source> {
    type Target = SpannedIter<'source, Token>;

    fn deref(&self) -> &Self::Target {
        &self.lexer
    }
}

impl<'source> DerefMut for State<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.lexer
    }
}
