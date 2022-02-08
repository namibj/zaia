use std::mem;

use cstree::{GreenNode, GreenNodeBuilder, NodeCache};

use super::{event::Event, kind::SyntaxKind, span::Span};
use crate::T;

pub struct Sink<'cache, 'source> {
    builder: GreenNodeBuilder<'cache, 'static>,
    tokens: &'source [(SyntaxKind, Span)],
    cursor: usize,
    events: Vec<Event>,
    source: &'source str,
}

impl<'cache, 'source> Sink<'cache, 'source> {
    pub fn new(
        cache: &'cache mut NodeCache<'static>,
        tokens: &'source [(SyntaxKind, Span)],
        events: Vec<Event>,
        source: &'source str,
    ) -> Self {
        Self {
            builder: GreenNodeBuilder::with_cache(cache),
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

    pub fn finish(mut self) -> GreenNode {
        let mut preceded_nodes = Vec::new();
        for idx in 0..self.events.len() {
            match mem::take(&mut self.events[idx]) {
                // Ignore tombstone events
                event @ Event::Enter { .. } if event.is_tombstone() => {},

                Event::Enter { kind, preceded_by } => {
                    preceded_nodes.push(kind);

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
                },

                Event::Exit => {
                    self.builder.finish_node();
                },

                Event::Token { kind, span } => {
                    self.token(kind, &self.source[span]);
                },
            }
        }

        self.builder.finish().0
    }
}
