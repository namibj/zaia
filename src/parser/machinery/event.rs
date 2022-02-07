use super::{kind::SyntaxKind, span::Span};
use crate::T;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Enter {
        kind: SyntaxKind,
        preceded_by: usize,
    },
    Exit,
    Token {
        kind: SyntaxKind,
        span: Span,
    },
}

impl Event {
    pub fn tombstone() -> Self {
        Self::Enter {
            kind: T![tombstone],
            preceded_by: 0,
        }
    }

    pub fn is_tombstone(self) -> bool {
        matches!(
            self,
            Self::Enter {
                kind: T![tombstone],
                preceded_by: 0,
            },
        )
    }
}

impl Default for Event {
    fn default() -> Self {
        Self::tombstone()
    }
}
