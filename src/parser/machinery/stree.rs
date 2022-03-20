use super::span::Span;

pub struct SyntaxKind {
    id: usize,
}

pub trait Language {
    type Kind;

    fn kind_from_raw(raw: SyntaxKind);
    fn kind_to_raw(kind: Self::Kind);
}

pub struct SyntaxToken {
    kind: SyntaxKind,
    span: Span,
}

pub struct SyntaxNode {
    kind: SyntaxKind,
    span: Span,
    children: Vec<Child>,
}

enum Child {
    Token(SyntaxToken),
    Node(SyntaxNode),
}