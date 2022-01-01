mod state;
mod token;
mod binding_power;

use super::syntax_tree::{
    Assign, BinaryExpr, BinaryOp, Do, Expr, ForGeneric, ForNumeric, Function, FunctionCall, Ident,
    If, IfChain, Label, Literal, NumLiteral, Repeat, Return, Stmt, SyntaxTree, Table, UnaryExpr,
    UnaryOp, While,
};
use state::State;
use crate::T;

pub fn parse(source: &str) -> (SyntaxTree, Vec<ariadne::Report>) {
    let mut state = State::new(source);
    let mut tree = SyntaxTree { block: Vec::new() };

    loop {
        match state.peek() {
            T![eof] => break,
            _ => todo!(),
        }
    }

    (tree, state.result())
}
