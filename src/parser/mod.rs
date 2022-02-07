mod control;
mod decl;
mod expr;
mod function;
mod item;
pub mod machinery;
mod simple_expr;
mod stmt;
mod syntax;
mod table;

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

#[cfg(test)]
mod tests {
    use std::fs;

    use insta::assert_debug_snapshot;
    use paste::paste;

    use super::parse;

    macro_rules! parse_and_verify {
        ($name:ident, $path:literal) => {
            paste! {
                #[test]
                fn [<parse_and_verify_ $name>]() {
                    let source = fs::read_to_string($path).unwrap();
                    let (syntax_tree, reports) = parse(&source);
                    assert!(reports.is_empty());
                    assert_debug_snapshot!(syntax_tree);
                }
            }
        };
    }

    parse_and_verify!(function, "test-files/function.lua");
    //parse_and_verify!(op_prec, "test-files/op_prec.lua");
    //parse_and_verify!(if, "test-files/if.lua");
    //parse_and_verify!(declare, "test-files/declare.lua");
    //parse_and_verify!(literal, "test-files/literal.lua");
    //parse_and_verify!(comment, "test-files/comment.lua");
    //parse_and_verify!(mixed, "test-files/mixed.lua");
}
