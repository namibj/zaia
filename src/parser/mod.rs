pub mod machinery;

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
            _ => parse_stmt(&mut state),
        }
    }

    marker.complete(&mut state, T![root]);
    state.finish()
}

fn parse_stmt(state: &mut State) {
    let marker = state.start();

    match state.at() {
        kind => state.error(
            state
                .new_error()
                .with_message("Unexpected token")
                .with_label(
                    state
                        .new_label()
                        .with_message(format!("Expecting start of statement but found {}", kind,)),
                )
                .finish(),
        ),
    }

    marker.complete(state, T![stmt]);
}
