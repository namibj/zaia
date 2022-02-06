use std::mem;

use cstree::{GreenNode, GreenNodeBuilder};
use logos::Logos;

use super::{event::Event, kind::SyntaxKind, marker::Marker, sink::Sink, span::Span};

pub struct State<'source> {
    tokens: Vec<(SyntaxKind, Span)>,
    cursor: usize,
    source: &'source str,
    events: Vec<Event>,
    errors: Vec<ariadne::Report<Span>>,
}

impl<'source> State<'source> {
    pub fn new(source: &'source str) -> Self {
        let mut tokens = Vec::new();
        tokens.extend(
            SyntaxKind::lexer(source)
                .spanned()
                .map(|(kind, range)| (kind, Span::from_range(range))),
        );

        State {
            tokens,
            cursor: 0,
            source,
            events: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn at(&self) -> SyntaxKind {
        self.tokens[self.cursor].0
    }

    pub fn span(&self) -> Span {
        self.tokens[self.cursor].1
    }

    pub fn start(&mut self) -> Marker {
        let pos = self.events.len();
        self.events.push(Event::tombstone());
        Marker::new(pos)
    }

    pub fn events(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }

    pub fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.at() == kind {
            self.bump();
            true
        } else {
            self.error(
                self.new_error()
                    .with_message("unexpected token")
                    .with_label(self.new_label().with_message(format!(
                        "expected token {} but found {}",
                        kind,
                        self.at()
                    )))
                    .finish(),
            );
            false
        }
    }

    pub fn error(&mut self, error: ariadne::Report<Span>) {
        self.errors.push(error);
    }

    pub fn new_error(&self) -> ariadne::ReportBuilder<Span> {
        ariadne::Report::build(ariadne::ReportKind::Error, (), self.span().start() as usize)
    }

    pub fn new_label(&self) -> ariadne::Label<Span> {
        ariadne::Label::new(self.span())
    }

    fn bump(&mut self) {
        self.events.push(Event::Token {
            kind: self.at(),
            span: self.span(),
        });

        self.cursor += 1;
    }

    fn eat_trivia(&mut self) {
        while self.at().is_trivia() {
            self.bump();
        }
    }

    pub fn source(&self, span: Span) -> &str {
        &self.source[span]
    }

    pub fn error_eat_until(&mut self, one_of: &[SyntaxKind]) -> Span {
        unimplemented!()
    }

    pub fn finish(self) -> (GreenNode, Vec<ariadne::Report<Span>>) {
        let tree = Sink::new(&self.tokens, self.events, self.source).finish();
        (tree, self.errors)
    }
}
