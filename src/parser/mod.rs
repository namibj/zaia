mod binding_power;
mod state;
mod token;

use super::syntax_tree::{
    Assign, BinaryExpr, BinaryOp, Do, Expr, ForGeneric, ForNumeric, Function, FunctionCall, Ident,
    If, IfChain, Label, Literal, NumLiteral, Repeat, Return, Stmt, SyntaxTree, Table, UnaryExpr,
    UnaryOp, While,
};
use crate::T;
use state::State;

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
