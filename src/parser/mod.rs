mod token;

use logos::{Logos, Lexer};
use super::ir::syntax_tree::{SyntaxTree,Stmt,Do,While,Repeat,If,IfChain,ForNumeric,ForGeneric,Return,Label,Assign,Expr,Table,FunctionCall,Function,Ident,UnaryExpr,BinaryExpr,UnaryOp,BinaryOp,Literal,NumLiteral};
use token::Token;

pub struct Parser<'source> {
    lexer: Lexer<'source, Token>,
}

impl<'source> Parser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            lexer: Token::lexer(source),
        }
    }
}
