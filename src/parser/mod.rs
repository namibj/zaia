pub mod machinery;

use std::str;

use cstree::GreenNode;
use machinery::{span::Span, state::State};

pub fn parse(source: &str) -> (GreenNode, Vec<ariadne::Report<Span>>) {
    let state = State::new(source);
    state.finish()
}
