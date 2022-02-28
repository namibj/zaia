use crate::{parser::machinery::kind::SyntaxKind, T, parser::machinery::cstree};

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

ast_node!(Root, T![root]);
ast_node!(Decl, T![decl_stmt]);
ast_node!(DeclTarget, T![decl_target]);
ast_node!(LiteralExpr, T![literal_expr]);
ast_node!(Assign, T![assign_stmt]);
ast_node!(Ident, T![ident]);
ast_node!(PrefixOp, T![prefix_op]);
ast_node!(BinaryOp, T![bin_op]);
ast_node!(ExprList, T![expr_list]);
ast_node!(FuncCall, T![func_call]);
ast_node!(Func, T![function]);
ast_node!(Table, T![table_expr]);
ast_node!(Break, T![break_stmt]);
ast_node!(Return, T![return_stmt]);
ast_node!(Block, T![block_stmt]);
ast_node!(While, T![while_stmt]);
ast_node!(Repeat, T![repeat_stmt]);
ast_node!(If, T![if_stmt]);
ast_node!(ElseChain, T![else_chain]);
ast_node!(ForNum, T![for_num_stmt]);
ast_node!(ForGen, T![for_gen_stmt]);
