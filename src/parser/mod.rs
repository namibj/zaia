mod state;
mod token;

use super::syntax_tree::{
    Assign, BinaryExpr, BinaryOp, Do, Expr, ForGeneric, ForNumeric, Function, FunctionCall, Ident,
    If, IfChain, Label, Literal, NumLiteral, Repeat, Return, Stmt, SyntaxTree, Table, UnaryExpr,
    UnaryOp, While,
};
use logos::Logos;
use state::State;
use token::Token;

pub fn parse(source: &str) -> (SyntaxTree, Vec<ariadne::Report>) {
    let mut state = State::new(source);
    let mut tree = SyntaxTree { block: Vec::new() };

    (tree, state.result())
}
