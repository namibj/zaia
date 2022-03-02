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

ast_node!(Root, T![root]);

impl Root {
    pub fn block(&self) -> impl Iterator<Item = Stmt> + '_ {
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
    Do(Do),
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
            T![do_stmt] => Self::Do(Do::cast(node)?),
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

impl FuncCall {
    pub fn target(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Expr> + '_> {
        Some(
            self.0
                .last_child()?
                .children()
                .cloned()
                .filter_map(Expr::cast),
        )
    }
}

ast_node!(Func, T![func_stmt]);

impl Func {
    pub fn target(&self) -> Option<SimpleExpr> {
        SimpleExpr::cast(self.0.first_child()?.clone())
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Ident> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .cloned()
                .filter_map(Ident::cast),
        )
    }

    pub fn block(&self) -> Option<Stmt> {
        Stmt::cast(self.0.last_child()?.clone())
    }
}

ast_node!(FuncExpr, T![func_expr]);

impl FuncExpr {
    pub fn args(&self) -> Option<impl Iterator<Item = Ident> + '_> {
        Some(
            self.0
                .first_child()?
                .children()
                .cloned()
                .filter_map(Ident::cast),
        )
    }

    pub fn block(&self) -> Option<Stmt> {
        Stmt::cast(self.0.last_child()?.clone())
    }
}

ast_node!(TableArray, T![table_array_elem]);

impl TableArray {
    pub fn value(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }
}

ast_node!(TableMap, T![table_map_elem]);

impl TableMap {
    pub fn field(&self) -> Option<Ident> {
        Ident::cast(self.0.first_child()?.clone())
    }

    pub fn value(&self) -> Option<Expr> {
        Expr::cast(self.0.last_child()?.clone())
    }
}

ast_node!(TableGeneric, T![table_generic_elem]);

impl TableGeneric {
    pub fn index(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }

    pub fn value(&self) -> Option<Expr> {
        Expr::cast(self.0.last_child()?.clone())
    }
}

ast_node!(Table, T![table_expr]);

impl Table {
    pub fn entries(&self) -> impl Iterator<Item = TableEntry> + '_ {
        self.0.children().cloned().filter_map(TableEntry::cast)
    }
}

pub enum TableEntry {
    Array(TableArray),
    Map(TableMap),
    Generic(TableGeneric),
}

impl TableEntry {
    fn cast(node: SyntaxNode) -> Option<Self> {
        Some(match node.kind() {
            T![table_array_elem] => Self::Array(TableArray::cast(node)?),
            T![table_map_elem] => Self::Map(TableMap::cast(node)?),
            T![table_generic_elem] => Self::Generic(TableGeneric::cast(node)?),
            _ => panic!(),
        })
    }
}

ast_node!(Break, T![break_stmt]);

ast_node!(Return, T![return_stmt]);

impl Return {
    pub fn exprs(&self) -> Option<impl Iterator<Item = Expr> + '_> {
        Some(
            self.0
                .first_child()?
                .children()
                .cloned()
                .filter_map(Expr::cast),
        )
    }
}

ast_node!(Do, T![do_stmt]);

impl Do {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt> + '_ {
        self.0.children().cloned().filter_map(Stmt::cast)
    }
}

ast_node!(While, T![while_stmt]);

impl While {
    pub fn cond(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        Some(
            self.0
                .last_child()?
                .children()
                .cloned()
                .filter_map(Stmt::cast),
        )
    }
}

ast_node!(Repeat, T![repeat_stmt]);

impl Repeat {
    pub fn cond(&self) -> Option<Expr> {
        Expr::cast(self.0.last_child()?.clone())
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        Some(
            self.0
                .first_child()?
                .children()
                .cloned()
                .filter_map(Stmt::cast),
        )
    }
}

ast_node!(If, T![if_stmt]);

impl If {
    pub fn cast_else(node: SyntaxNode) -> Option<Self> {
        if node.kind() == T![elseif] {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn cond(&self) -> Option<Expr> {
        Expr::cast(self.0.first_child()?.clone())
    }

    pub fn stmts(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .cloned()
                .filter_map(Stmt::cast),
        )
    }

    pub fn else_chain(&self) -> Option<ElseChain> {
        ElseChain::cast(self.0.last_child()?.clone())
    }
}

ast_node!(ElseChain, T![else_chain]);

impl ElseChain {
    pub fn else_block(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        let token = self.0.first_token()?;

        if token.kind() == T![else] {
            Some(
                self.0
                    .first_child()?
                    .children()
                    .cloned()
                    .filter_map(Stmt::cast),
            )
        } else {
            None
        }
    }

    pub fn elseif_block(&self) -> Option<If> {
        let token = self.0.first_token()?;

        if token.kind() == T![elseif] {
            If::cast_else(self.0.first_child()?.clone())
        } else {
            None
        }
    }
}

ast_node!(ForNum, T![for_num_stmt]);

impl ForNum {
    pub fn counter(&self) -> Option<(Ident, Expr)> {
        let mut children = self.0.children().cloned();
        let name = Ident::cast(children.next()?)?;
        let value = Expr::cast(children.next()?)?;
        Some((name, value))
    }

    pub fn end(&self) -> Option<Expr> {
        Expr::cast(self.0.children().skip(2).next()?.clone())
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        Some(
            self.0
                .last_child()?
                .children()
                .cloned()
                .filter_map(Stmt::cast),
        )
    }
}

ast_node!(ForGen, T![for_gen_stmt]);

impl ForGen {
    pub fn targets(&self) -> Option<impl Iterator<Item = Ident> + '_> {
        Some(
            self.0
                .first_child()?
                .children()
                .cloned()
                .filter_map(Ident::cast),
        )
    }

    pub fn values(&self) -> Option<impl Iterator<Item = Expr> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .cloned()
                .filter_map(Expr::cast),
        )
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt> + '_> {
        Some(
            self.0
                .last_child()?
                .children()
                .cloned()
                .filter_map(Stmt::cast),
        )
    }
}
