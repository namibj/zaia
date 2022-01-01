mod token;
mod state;

use logos::Logos;
use token::Token;
use super::syntax_tree::{SyntaxTree,Stmt,Do,While,Repeat,If,IfChain,ForNumeric,ForGeneric,Return,Label,Assign,Expr,Table,FunctionCall,Function,Ident,UnaryExpr,BinaryExpr,UnaryOp,BinaryOp,Literal,NumLiteral};
use state::State;

pub fn parse(source: &str) -> SyntaxTree {
    let mut state = State::new(source);
    let mut tree = SyntaxTree {
        block: Vec::new(),
    };

    tree
}
