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
use either::Either;

pub fn parse(source: &str) -> (SyntaxTree, Vec<ariadne::Report>) {
    let mut state = State::new(source);
    let mut tree = SyntaxTree { block: Vec::new() };

    loop {
        match state.peek() {
            T![eof] => break,
            _ => {
                let stmt = parse_stmt(&mut state);
                tree.block.push(stmt);
            }
        }
    }

    (tree, state.result())
}

fn parse_stmt(state: &mut State) -> Stmt {
    loop {
        match state.peek() {
            T![invalid] => {
                let item = parse_expr(state);
                return Stmt::Expr(item);
            }

            T![::] => {
                let item = parse_label(state);
                return Stmt::Label(item);
            }

            T![do] => {
                let item = parse_do(state);
                return Stmt::Do(item);
            }

            T![while] => {
                let item = parse_while(state);
                return Stmt::While(item);
            }

            T![repeat] => {
                let item = parse_repeat(state);
                return Stmt::Repeat(item);
            }

            T![if] => {
                let item = parse_if(state);
                return Stmt::If(item);
            }

            T![for] => {
                match parse_for(state) {
                    Either::Left(numeric) => return Stmt::ForNumeric(numeric),
                    Either::Right(generic) => return Stmt::ForGeneric(generic),
                }
            }

            T![return] => {
                let item = parse_return(state);
                return Stmt::Return(item);
            }

            T![break] => {
                parse_break(state);
                return Stmt::Break;
            }

            _ => todo!(),
        }
    }
}

fn parse_expr(state: &mut State) -> Expr {
    todo!()
}

fn parse_label(state: &mut State) -> Label {
    todo!()
}

fn parse_do(state: &mut State) -> Do {
    todo!()
}

fn parse_while(state: &mut State) -> While {
    todo!()
}

fn parse_repeat(state: &mut State) -> Repeat {
    todo!()
}

fn parse_if(state: &mut State) -> If {
    todo!()
}

fn parse_for(state: &mut State) -> Either<ForNumeric, ForGeneric> {
    todo!()
}

fn parse_for_numeric(state: &mut State) -> ForNumeric {
    todo!()
}

fn parse_for_generic(state: &mut State) -> ForGeneric {
    todo!()
}

fn parse_return(state: &mut State) -> Return {
    todo!()
}

fn parse_break(state: &mut State) {
    todo!()
}

fn parse_ident(state: &mut State) -> Ident {
    todo!()
}

fn parse_unary_expr(state: &mut State) -> UnaryExpr {
    todo!()
}

fn parse_binary_expr(state: &mut State) -> BinaryExpr {
    todo!()
}

fn parse_function(state: &mut State) -> Function {
    todo!()
}

fn parse_literal(state: &mut State) -> Literal {
    todo!()
}

fn parse_function_call(state: &mut State) -> FunctionCall {
    todo!()
}

fn parse_table(state: &mut State) -> Table {
    todo!()
}

fn parse_assign(state: &mut State) -> Assign {
    todo!()
}
