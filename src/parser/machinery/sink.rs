use std::mem;

use cstree::{GreenNode, GreenNodeBuilder};

use super::{event::Event, kind::SyntaxKind, span::Span};
use crate::T;

pub struct Sink<'source> {
    builder: GreenNodeBuilder<'static, 'static>,
    tokens: &'source [(SyntaxKind, Span)],
    cursor: usize,
    events: Vec<Event>,
    source: &'source str,
}

impl<'source> Sink<'source> {
    pub fn new(
        tokens: &'source [(SyntaxKind, Span)],
        events: Vec<Event>,
        source: &'source str,
    ) -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            tokens,
            cursor: 0,
            events,
            source,
        }
    }

    fn token(&mut self, kind: SyntaxKind, text: &str) {
        self.cursor += 1;
        self.builder.token(kind.into(), text);
    }

    fn eat_trivia(&mut self) {
        while let Some((kind, span)) = self.tokens.get(self.cursor) {
            if !kind.is_trivia() {
                break;
            }

            self.token(*kind, &self.source[*span]);
        }
    }

    pub(crate) fn finish(mut self) -> GreenNode {
        let mut preceded_nodes = Vec::new();
        for idx in 0..self.events.len() {
            match mem::take(&mut self.events[idx]) {
                // Ignore tombstone events
                event @ Event::Enter { .. } if event.is_tombstone() => {},

                Event::Enter { kind, preceded_by } => {
                    preceded_nodes.push(kind);

                    if kind != T![root] {
                        self.eat_trivia();
                    }

                    let (mut idx, mut preceded_by) = (idx, preceded_by);
                    while preceded_by > 0 {
                        idx += preceded_by;

                        preceded_by = match mem::take(&mut self.events[idx]) {
                            Event::Enter { kind, preceded_by } => {
                                if kind != T![tombstone] {
                                    preceded_nodes.push(kind);
                                }

                                preceded_by
                            },

                            _ => unreachable!(),
                        }
                    }

                    for kind in preceded_nodes.drain(..).rev() {
                        self.builder.start_node(kind.into());
                    }

                    // Note: We eat trivia *after* entering all the required nodes
                    //       since otherwise this'll make us eat whitespace before
                    //       we can open up the root node, which is bad
                    self.eat_trivia();
                },

                Event::Exit => {
                    self.builder.finish_node();
                    self.eat_trivia();
                },

                Event::Token { kind, span } => {
                    self.eat_trivia();
                    self.token(kind, &self.source[span]);
                },
            }
        }

        self.builder.finish().0
    }
}
