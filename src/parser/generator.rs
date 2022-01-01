use super::super::ir::syntax_tree::{SyntaxTree,Stmt,Do,While,Repeat,If,IfChain,ForNumeric,ForGeneric,Return,Label,Assign,Expr,Table,FunctionCall,Function,Ident,UnaryExpr,BinaryExpr,UnaryOp,BinaryOp,Literal,NumLiteral};

pub struct Generator {
    tree: SyntaxTree,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            tree: SyntaxTree {
                block: Vec::new(),
            },
        }
    }
}
