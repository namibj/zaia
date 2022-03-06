use crate::{
    parser::machinery::{cstree, cstree::interning::TokenInterner, kind::SyntaxKind},
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
        pub struct $name<'cst>(&'cst SyntaxNode);
        impl<'cst> $name<'cst> {
            fn cast(node: &'cst SyntaxNode) -> Option<Self> {
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

impl<'cst> Root<'cst> {
    pub fn block(&self) -> impl Iterator<Item = Stmt<'cst>> + '_ {
        self.0.children().filter_map(Stmt::cast)
    }
}

pub enum Stmt<'cst> {
    Decl(Decl<'cst>),
    Assign(Assign<'cst>),
    Func(Func<'cst>),
    Expr(Expr<'cst>),
    Break(Break<'cst>),
    Return(Return<'cst>),
    Do(Do<'cst>),
    While(While<'cst>),
    Repeat(Repeat<'cst>),
    If(If<'cst>),
    ForNum(ForNum<'cst>),
    ForGen(ForGen<'cst>),
}

impl<'cst> Stmt<'cst> {
    fn cast(node: &'cst SyntaxNode) -> Option<Self> {
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
            _ => Self::Expr(Expr::cast(node)?),
        })
    }
}

pub struct Expr<'cst>(&'cst SyntaxNode);

impl<'cst> Expr<'cst> {
    fn cast(node: &'cst SyntaxNode) -> Option<Self> {
        todo!()
    }
}

ast_node!(Decl, T![decl_stmt]);

impl<'cst> Decl<'cst> {
    pub fn targets(&self) -> impl Iterator<Item = DeclTarget<'cst>> + '_ {
        self.0.children().filter_map(DeclTarget::cast)
    }
}

ast_node!(DeclTarget, T![decl_target]);

impl<'cst> DeclTarget<'cst> {
    pub fn name(&self) -> Option<Ident<'cst>> {
        self.0.first_child().and_then(Ident::cast)
    }

    pub fn modifier(&self) -> Option<DeclModifier> {
        match self.0.last_token() {
            Some(token) => DeclModifier::cast(token),
            None => None,
        }
    }
}

pub enum DeclModifier {
    Const,
    Close,
}

impl DeclModifier {
    fn cast(token: &SyntaxToken) -> Option<Self> {
        Some(match token.kind() {
            T![const] => Self::Const,
            T![close] => Self::Close,
            _ => return None,
        })
    }
}

ast_node!(LiteralExpr, T![literal_expr]);

impl<'cst> LiteralExpr<'cst> {
    pub fn value(&self) {
        todo!()
    }
}

ast_node!(Assign, T![assign_stmt]);

impl<'cst> Assign<'cst> {
    pub fn targets(&self) -> Option<impl Iterator<Item = Expr<'cst>> + '_> {
        Some(self.0.first_child()?.children().filter_map(Expr::cast))
    }

    pub fn values(&self) -> Option<impl Iterator<Item = Expr<'cst>> + '_> {
        Some(self.0.last_child()?.children().filter_map(Expr::cast))
    }
}

ast_node!(Ident, T![ident]);

impl<'cst> Ident<'cst> {
    pub fn name<'a>(&self, interner: &'a TokenInterner) -> Option<&'a str> {
        Some(self.0.first_token()?.resolve_text(interner))
    }
}

ast_node!(PrefixOp, T![prefix_op]);

impl<'cst> PrefixOp<'cst> {
    pub fn op(&self) -> Option<PrefixOperator> {
        PrefixOperator::cast(self.0.first_token()?)
    }

    pub fn rhs(&self) -> Option<Expr> {
        self.0.first_child().and_then(Expr::cast)
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
        todo!()
    }
}

ast_node!(BinaryOp, T![bin_op]);

impl<'cst> BinaryOp<'cst> {
    pub fn op(&self) -> Option<BinaryOperator> {
        BinaryOperator::cast(self.0.first_token()?)
    }

    pub fn lhs(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn rhs(&self) -> Option<Expr<'cst>> {
        self.0.last_child().and_then(Expr::cast)
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
        todo!()
    }
}

ast_node!(FuncCall, T![func_call]);

impl<'cst> FuncCall<'cst> {
    pub fn target(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Expr<'cst>> + '_> {
        Some(self.0.last_child()?.children().filter_map(Expr::cast))
    }
}

ast_node!(Func, T![func_stmt]);

impl<'cst> Func<'cst> {
    pub fn target(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn args(&self) -> Option<impl Iterator<Item = Ident<'cst>> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .filter_map(Ident::cast),
        )
    }

    pub fn block(&self) -> Option<Stmt<'cst>> {
        self.0.last_child().and_then(Stmt::cast)
    }
}

ast_node!(FuncExpr, T![func_expr]);

impl<'cst> FuncExpr<'cst> {
    pub fn args(&self) -> Option<impl Iterator<Item = Ident<'cst>> + '_> {
        Some(self.0.first_child()?.children().filter_map(Ident::cast))
    }

    pub fn block(&self) -> Option<Stmt<'cst>> {
        self.0.last_child().and_then(Stmt::cast)
    }
}

ast_node!(TableArray, T![table_array_elem]);

impl<'cst> TableArray<'cst> {
    pub fn value(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }
}

ast_node!(TableMap, T![table_map_elem]);

impl<'cst> TableMap<'cst> {
    pub fn field(&self) -> Option<Ident<'cst>> {
        self.0.first_child().and_then(Ident::cast)
    }

    pub fn value(&self) -> Option<Expr<'cst>> {
        self.0.last_child().and_then(Expr::cast)
    }
}

ast_node!(TableGeneric, T![table_generic_elem]);

impl<'cst> TableGeneric<'cst> {
    pub fn index(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn value(&self) -> Option<Expr<'cst>> {
        self.0.last_child().and_then(Expr::cast)
    }
}

ast_node!(Table, T![table_expr]);

impl<'cst> Table<'cst> {
    pub fn entries(&self) -> impl Iterator<Item = TableEntry<'cst>> + '_ {
        self.0.children().filter_map(TableEntry::cast)
    }
}

pub enum TableEntry<'cst> {
    Array(TableArray<'cst>),
    Map(TableMap<'cst>),
    Generic(TableGeneric<'cst>),
}

impl<'cst> TableEntry<'cst> {
    fn cast(node: &'cst SyntaxNode) -> Option<Self> {
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

impl<'cst> Return<'cst> {
    pub fn exprs(&self) -> Option<impl Iterator<Item = Expr<'cst>> + '_> {
        Some(self.0.first_child()?.children().filter_map(Expr::cast))
    }
}

ast_node!(Do, T![do_stmt]);

impl<'cst> Do<'cst> {
    pub fn stmts(&self) -> impl Iterator<Item = Stmt<'cst>> + '_ {
        self.0.children().filter_map(Stmt::cast)
    }
}

ast_node!(While, T![while_stmt]);

impl<'cst> While<'cst> {
    pub fn cond(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        Some(self.0.last_child()?.children().filter_map(Stmt::cast))
    }
}

ast_node!(Repeat, T![repeat_stmt]);

impl<'cst> Repeat<'cst> {
    pub fn cond(&self) -> Option<Expr<'cst>> {
        self.0.last_child().and_then(Expr::cast)
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        Some(self.0.first_child()?.children().filter_map(Stmt::cast))
    }
}

ast_node!(If, T![if_stmt]);

impl<'cst> If<'cst> {
    pub fn cast_else(node: &'cst SyntaxNode) -> Option<Self> {
        if node.kind() == T![elseif] {
            Some(Self(node))
        } else {
            None
        }
    }

    pub fn cond(&self) -> Option<Expr<'cst>> {
        self.0.first_child().and_then(Expr::cast)
    }

    pub fn stmts(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .filter_map(Stmt::cast),
        )
    }

    pub fn else_chain(&self) -> Option<ElseChain<'cst>> {
        self.0.last_child().and_then(ElseChain::cast)
    }
}

ast_node!(ElseChain, T![else_chain]);

impl<'cst> ElseChain<'cst> {
    pub fn else_block(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        let token = self.0.first_token()?;

        if token.kind() == T![else] {
            Some(self.0.first_child()?.children().filter_map(Stmt::cast))
        } else {
            None
        }
    }

    pub fn elseif_block(&self) -> Option<If<'cst>> {
        let token = self.0.first_token()?;

        if token.kind() == T![elseif] {
            If::cast_else(self.0.first_child()?)
        } else {
            None
        }
    }
}

ast_node!(ForNum, T![for_num_stmt]);

impl<'cst> ForNum<'cst> {
    pub fn counter(&self) -> Option<(Ident<'cst>, Expr<'cst>)> {
        let mut children = self.0.children();
        let name = children.next().and_then(Ident::cast)?;
        let value = children.next().and_then(Expr::cast)?;
        Some((name, value))
    }

    pub fn end(&self) -> Option<Expr<'cst>> {
        self.0.children().skip(2).next().and_then(Expr::cast)
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        Some(self.0.last_child()?.children().filter_map(Stmt::cast))
    }
}

ast_node!(ForGen, T![for_gen_stmt]);

impl<'cst> ForGen<'cst> {
    pub fn targets(&self) -> Option<impl Iterator<Item = Ident<'cst>> + '_> {
        Some(self.0.first_child()?.children().filter_map(Ident::cast))
    }

    pub fn values(&self) -> Option<impl Iterator<Item = Expr<'cst>> + '_> {
        Some(
            self.0
                .children()
                .skip(1)
                .next()?
                .children()
                .filter_map(Expr::cast),
        )
    }

    pub fn block(&self) -> Option<impl Iterator<Item = Stmt<'cst>> + '_> {
        Some(self.0.last_child()?.children().filter_map(Stmt::cast))
    }
}
