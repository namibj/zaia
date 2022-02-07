mod control;
mod decl;
mod expr;
mod function;
mod item;
pub mod machinery;
mod simple_expr;
mod stmt;
mod syntax;

use std::ops::{Deref, DerefMut};

use cstree::GreenNode;
use machinery::{kind::SyntaxKind, marker::Marker, span::Span, state::State};

use crate::T;

struct Parser<'source> {
    state: State<'source>,
}

impl<'source> Parser<'source> {
    fn new(source: &'source str) -> Self {
        Self {
            state: State::new(source),
        }
    }

    fn root(&mut self) {
        let marker = self.start();
        self.r_items();
        marker.complete(self, T![root]);
    }

    fn run(mut self) -> (GreenNode, Vec<ariadne::Report<Span>>) {
        self.root();
        self.state.finish()
    }
}

impl<'source> Deref for Parser<'source> {
    type Target = State<'source>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'source> DerefMut for Parser<'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

pub fn parse(source: &str) -> (GreenNode, Vec<ariadne::Report<Span>>) {
    Parser::new(source).run()
}
