use cstree::GreenNode;

use crate::{parser::machinery::kind::SyntaxKind, T};

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
