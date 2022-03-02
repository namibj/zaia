use crate::{
    parser::machinery::{cstree, kind::SyntaxKind},
    T,
};

impl From<SyntaxKind> for cstree::SyntaxKind {
    fn from(token: SyntaxKind) -> Self {
        Self(token as u16)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Lang {}

impl cstree::Language for Lang {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: cstree::SyntaxKind) -> Self::Kind {
        debug_assert!(raw.0 < T![__LAST] as u16);
        unsafe { std::mem::transmute(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> cstree::SyntaxKind {
        kind.into()
    }
}

pub type SyntaxNode = cstree::SyntaxNode<Lang>;
pub type SyntaxToken = cstree::SyntaxToken<Lang>;
pub type SyntaxElement = cstree::NodeOrToken<SyntaxNode, SyntaxToken>;

macro_rules! ast_node {
    ($name:ident, $kind:expr) => {
        #[derive(PartialEq, Eq, Hash)]
        #[repr(transparent)]
        pub struct $name(SyntaxNode);
        impl $name {
            #[allow(unused)]
            fn cast(node: SyntaxNode) -> Option<Self> {
                if node.kind() == $kind {
                    Some(Self(node))
                } else {
                    None
                }
            }
        }
    };
}

fn expr_list(node: Option<&SyntaxNode>) -> impl Iterator<Item = Expr> + '_ {
    node.into_iter()
        .flat_map(|node| node.children().cloned().filter_map(Expr::cast))
}

ast_node!(Root, T![root]);

impl Root {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> + '_ {
        self.0.children().cloned().filter_map(Stmt::cast)
    }
}

pub enum Stmt {
    Decl(Decl),
    Assign(Assign),
    Func(Func),
    SimpleExpr(SimpleExpr),
    Break(Break),
    Return(Return),
    Block(Block),
    While(While),
    Repeat(Repeat),
    If(If),
    ForNum(ForNum),
    ForGen(ForGen),
}

impl Stmt {
    fn cast(node: SyntaxNode) -> Option<Self> {
        Some(match node.kind() {
            T![decl_stmt] => Self::Decl(Decl::cast(node)?),
            T![assign_stmt] => Self::Assign(Assign::cast(node)?),
            T![func_stmt] => Self::Func(Func::cast(node)?),
            T![break_stmt] => Self::Break(Break::cast(node)?),
            T![return_stmt] => Self::Return(Return::cast(node)?),
            T![block_stmt] => Self::Block(Block::cast(node)?),
            T![while_stmt] => Self::While(While::cast(node)?),
            T![repeat_stmt] => Self::Repeat(Repeat::cast(node)?),
            T![if_stmt] => Self::If(If::cast(node)?),
            T![for_num_stmt] => Self::ForNum(ForNum::cast(node)?),
            T![for_gen_stmt] => Self::ForGen(ForGen::cast(node)?),
            kind if SimpleExpr::TOKENS.contains(&kind) => Self::SimpleExpr(SimpleExpr::cast(node)?),
            _ => unreachable!(),
        })
    }
}

pub enum SimpleExpr {}

impl SimpleExpr {
    const TOKENS: &'static [SyntaxKind] = &[];

    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            _ => unreachable!(),
        }
    }
}

pub enum Expr {}

impl Expr {
    const TOKENS: &'static [SyntaxKind] = &[];

    fn cast(node: SyntaxNode) -> Option<Self> {
        match node.kind() {
            _ => unreachable!(),
        }
    }
}

ast_node!(Decl, T![decl_stmt]);

ast_node!(DeclTarget, T![decl_target]);

ast_node!(LiteralExpr, T![literal_expr]);

ast_node!(Assign, T![assign_stmt]);

ast_node!(Ident, T![ident]);

ast_node!(PrefixOp, T![prefix_op]);

impl PrefixOp {
    pub fn op(&self) -> Option<PrefixOperator> {
        PrefixOperator::cast(self.0.first_token()?)
    }

    pub fn rhs(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }
}

pub enum PrefixOperator {
    None,
    Neg,
    Not,
    Len,
    BitNot,
}

impl PrefixOperator {
    fn cast(node: &SyntaxToken) -> Option<Self> {
        match node.kind() {
            _ => unreachable!(),
        }
    }
}

ast_node!(BinaryOp, T![bin_op]);

impl BinaryOp {
    pub fn op(&self) -> Option<BinaryOperator> {
        BinaryOperator::cast(self.0.first_token()?)
    }

    pub fn lhs(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }

    pub fn rhs(&self) -> Option<Expr> {
        Expr::cast(self.0.last_child()?.clone())
    }
}

pub enum BinaryOperator {
    And,
    Or,
    Add,
    Sub,
    Mul,
    Div,
    IntDiv,
    Exp,
    Mod,
    BitAnd,
    BitOr,
    LShift,
    RShift,
    Eq,
    BitXor,
    NEq,
    LEq,
    GEq,
    Gt,
    Lt,
    Property,
    Method,
    Concat,
}

impl BinaryOperator {
    fn cast(node: &SyntaxToken) -> Option<Self> {
        match node.kind() {
            _ => unreachable!(),
        }
    }
}

ast_node!(FuncCall, T![func_call]);

ast_node!(Func, T![function]);

ast_node!(Table, T![table_expr]);

ast_node!(Break, T![break_stmt]);

ast_node!(Return, T![return_stmt]);

impl Return {
    pub fn exprs(&self) -> impl Iterator<Item = Expr> + '_ {
        expr_list(self.0.first_child())
    }
}

ast_node!(Block, T![block_stmt]);

impl Block {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> + '_ {
        self.0.children().cloned().filter_map(Stmt::cast)
    }
}

ast_node!(While, T![while_stmt]);

ast_node!(Repeat, T![repeat_stmt]);

ast_node!(If, T![if_stmt]);

ast_node!(ElseChain, T![else_chain]);

ast_node!(ForNum, T![for_num_stmt]);

ast_node!(ForGen, T![for_gen_stmt]);
