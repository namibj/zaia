use std::ops::Range;

pub struct Token {
    kind: TokenKind,
    span: Range<usize>,
}

pub enum TokenKind {
    // Catch-all required by the lexer
    Invalid,

    // Keywords
    And,
    Local,
    Then,
    Break,
    For,
    Do,
    Function,
    Until,
    Else,
    Goto,
    Or,
    While,
    ElseIf,
    If,
    Repeat,
    End,
    In,
    Return,

    // Literals
    Nil,
    False,
    True,
    String,
    Integer,
    HexInteger,
    Float,
    FloatExp,
    HexFloat,
    HexFloatExp,
    HexFloatBinaryExp,
    
    // Unary operations
    Not,
    BitNot,

    // Binary operators
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Exp,
    Len,
    BitAnd,
    Xor,
    BitOr,
    LeftShift,
    RightShift,
    DivFloor,
    Equals,
    NotEquals,
    LesserOrEquals,
    GreaterOrEquals,
    Lesser,
    Greater,
    Assign,

    // Other
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftBracket,
    RightBracket,
    Colon,
    DoubleColon,
    Semicolon,
    Comma,
    Dot,
    DoubleDot,
    TripleDot,
    Comment,
}
