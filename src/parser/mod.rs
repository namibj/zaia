pub mod machinery;
mod rules;

use std::str;

use cstree::GreenNode;
use machinery::{kind::SyntaxKind, marker::Marker, span::Span, state::State};

use crate::T;

pub fn parse(source: &str) -> (GreenNode, Vec<ariadne::Report<Span>>) {
    let mut state = State::new(source);
    let marker = state.start();

    loop {
        match state.at() {
            T![eof] => break,
            _ => rules::stmt::parse_stmt(&mut state),
        }
    }

    marker.complete(&mut state, T![root]);
    state.finish()
}
