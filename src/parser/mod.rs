mod assign;
mod control;
mod expr;
mod function;
mod item;
pub mod machinery;
mod simple_expr;
mod stmt;
mod syntax;
mod table;

use std::ops::{Deref, DerefMut};

use cstree::NodeCache;
use machinery::{span::Span, state::State};
use syntax::SyntaxNode;

use crate::T;

struct Parser<'cache, 'source> {
    state: State<'cache, 'source>,
}

impl<'cache, 'source> Parser<'cache, 'source> {
    fn new(cache: &'cache mut NodeCache<'static>, source: &'source str) -> Self {
        Self {
            state: State::new(cache, source),
        }
    }

    fn root(&mut self) {
        let marker = self.start();
        self.r_items();
        marker.complete(self, T![root]);
    }

    fn run(mut self) -> (SyntaxNode, Vec<ariadne::Report<Span>>) {
        self.root();
        let (root, reports) = self.state.finish();
        (SyntaxNode::new_root(root), reports)
    }
}

impl<'cache, 'source> Deref for Parser<'cache, 'source> {
    type Target = State<'cache, 'source>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'cache, 'source> DerefMut for Parser<'cache, 'source> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

pub fn parse(
    cache: &mut NodeCache<'static>,
    source: &str,
) -> (SyntaxNode, Vec<ariadne::Report<Span>>) {
    Parser::new(cache, source).run()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use cstree::NodeCache;
    use insta::assert_snapshot;
    use paste::paste;

    use super::{parse, syntax::SyntaxNode};

    fn syntax_tree_debug(cache: &NodeCache<'static>, node: &SyntaxNode) -> String {
        node.debug(cache.interner(), true)
    }

    macro_rules! parse_and_verify {
        ($name:ident, $path:literal) => {
            paste! {
                #[test]
                fn [<parse_and_verify_ $name>]() {
                    let mut cache = NodeCache::new();
                    let source = fs::read_to_string($path).unwrap();
                    let (syntax_tree, reports) = parse(&mut cache, &source);
                    let syntax_tree_debug = syntax_tree_debug(&cache, &syntax_tree);
                    assert!(reports.is_empty());
                    assert_snapshot!(syntax_tree_debug);
                }
            }
        };
    }

    parse_and_verify!(function, "test-files/function.lua");
    parse_and_verify!(op_prec, "test-files/op_prec.lua");
    parse_and_verify!(if, "test-files/if.lua");
    parse_and_verify!(declare, "test-files/declare.lua");
    parse_and_verify!(literal, "test-files/literal.lua");
    parse_and_verify!(comment, "test-files/comment.lua");
    parse_and_verify!(mixed, "test-files/mixed.lua");
}
