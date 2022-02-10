use std::ops::Not;

use cstree::{GreenNode, NodeCache};
use logos::Logos;

use super::{event::Event, kind::SyntaxKind, marker::Marker, sink::Sink, span::Span};
use crate::T;

pub struct State<'cache, 'source> {
    cache: &'cache mut NodeCache<'static>,
    tokens: Vec<(SyntaxKind, Span)>,
    cursor: usize,
    source: &'source str,
    events: Vec<Event>,
    reports: Vec<ariadne::Report<Span>>,
}

impl<'cache, 'source> State<'cache, 'source> {
    pub fn new(cache: &'cache mut NodeCache<'static>, source: &'source str) -> Self {
        let mut tokens = Vec::new();
        tokens.extend(
            SyntaxKind::lexer(source)
                .spanned()
                .map(|(kind, range)| (kind, Span::from_range(range))),
        );

        tokens.push((T![eof], Span::from_range(0..0)));
        let estimated_events = source.len() / 4;

        State {
            cache,
            tokens,
            cursor: 0,
            source,
            events: Vec::with_capacity(estimated_events),
            reports: Vec::new(),
        }
    }

    pub fn at(&self) -> SyntaxKind {
        self.tokens[self.cursor].0
    }

    pub fn peek(&self) -> SyntaxKind {
        self.tokens[self.cursor + 1..]
            .iter()
            .find_map(|(t, _)| t.is_trivia().not().then(|| *t))
            .unwrap()
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
            self.report(
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

    pub fn report(&mut self, error: ariadne::Report<Span>) {
        self.reports.push(error);
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

    pub fn source(&self, span: Span) -> &str {
        &self.source[span]
    }

    pub fn error_eat_until(&mut self, one_of: &[SyntaxKind]) -> Span {
        let marker = self.start();
        let mut last_span = self.span();
        while !one_of.contains(&self.at()) {
            self.bump();
            last_span = self.span();
        }

        marker.complete(self, T![invalid]);
        last_span
    }

    pub fn finish(self) -> (GreenNode, Vec<ariadne::Report<Span>>) {
        let tree = Sink::new(self.cache, &self.tokens, self.events, self.source).finish();
        (tree, self.reports)
    }
}
