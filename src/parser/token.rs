use std::fmt::{self, Display};

use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    // Miscellaneous
    #[regex("--", skip_comment)]
    #[regex(r"[ \n\t\f\r;]+", logos::skip)]
    #[error]
    Invalid,

    Eof,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 3)]
    Ident,

    // Character operators
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("^")]
    Caret,

    #[token("#")]
    Hash,

    #[token("&")]
    Ampersand,

    #[token("|")]
    Pipe,

    #[token("~")]
    Tilde,

    #[token("<<")]
    DLAngle,

    #[token(">>")]
    DRAngle,

    #[token("==")]
    Eq,

    #[token("~=")]
    NotEq,

    #[token("<=")]
    LEq,

    #[token(">=")]
    GEq,

    #[token("<")]
    LAngle,

    #[token(">")]
    RAngle,

    #[token("=")]
    Assign,

    #[token("//")]
    DSlash,

    #[token(".")]
    Dot,

    #[token("..")]
    DDot,

    // Keywords
    #[token("local")]
    Local,

    #[token("function")]
    Function,

    #[token("end")]
    End,

    #[token("in")]
    In,

    #[token("then")]
    Then,

    #[token("break")]
    Break,

    #[token("for")]
    For,

    #[token("do")]
    Do,

    #[token("until")]
    Until,

    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("elseif")]
    ElseIf,

    #[token("if")]
    If,

    #[token("repeat")]
    Repeat,

    #[token("return")]
    Return,

    #[token("not")]
    Not,

    #[token("or")]
    Or,

    #[token("and")]
    And,

    #[token("<const>")]
    Const,

    #[token("<close>")]
    Close,

    // Literals
    #[token("nil")]
    Nil,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[regex(r#""(\\[\\"]|[^"])*""#)]
    #[regex(r#"'(\\[\\']|[^'])*'"#)]
    String,

    #[regex(r"\[=*\[", long_string)]
    LongString,

    #[regex(r"[0-9]+", priority = 2)]
    Int,

    #[regex(r"0x[0-9a-fA-F]+")]
    HexInt,

    #[regex(r"[0-9]+(\.[0-9]+)?([eE][+-]?[0-9]+)?")]
    Float,

    #[regex(r"0x[0-9a-fA-F]*\.[0-9a-fA-F]+([pP][+-][0-9a-fA-F]+)?")]
    HexFloat,

    // Grouping
    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LCurly,

    #[token("}")]
    RCurly,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token(":")]
    Colon,

    #[token("::")]
    DColon,

    #[token(",")]
    Comma,

    #[token("...")]
    TDot,
}

fn long_string(lexer: &mut Lexer<Token>) {
    let delim_len = lexer.slice().len();
    let rem = lexer.remainder();

    for (i, _) in rem.char_indices() {
        if is_long_delimiter(&rem[i..i + delim_len], ']') {
            lexer.bump(i + delim_len);
            return;
        }
    }

    unreachable!()
}

fn skip_comment(lexer: &mut Lexer<Token>) -> logos::Skip {
    let rem = lexer.remainder();

    if let Some(delim_len) = starts_with_long_delimiter(rem, '[') {
        lexer.bump(delim_len);
        skip_long_comment(lexer, delim_len);
        logos::Skip
    } else {
        for (i, _) in rem.char_indices() {
            let curr = &rem[i..];
            if curr.starts_with("\r\n") {
                lexer.bump(i - 1);
                return logos::Skip;
            }

            if curr.starts_with('\n') {
                lexer.bump(i);
                return logos::Skip;
            }
        }

        unreachable!();
    }
}

fn skip_long_comment(lexer: &mut Lexer<Token>, delim_len: usize) {
    let rem = lexer.remainder();

    for (i, _) in rem.char_indices() {
        if is_long_delimiter(&rem[i..i + delim_len], ']') {
            lexer.bump(i + delim_len);
            return;
        }
    }

    unreachable!()
}

fn starts_with_long_delimiter(slice: &str, delim: char) -> Option<usize> {
    for (i, _) in slice.char_indices() {
        if is_long_delimiter(&slice[..i], delim) {
            return Some(i);
        }
    }

    None
}

fn is_long_delimiter(slice: &str, delim: char) -> bool {
    if slice.len() < 2 || !slice.starts_with(delim) || !slice.ends_with(delim) {
        return false;
    }

    slice.chars().filter(|c| *c == '=').count() + 2 == slice.len()
}

#[macro_export]
macro_rules! T {
    [invalid] => { $crate::parser::token::Token::Invalid };
    [eof] => { $crate::parser::token::Token::Eof };
    [ident] => { $crate::parser::token::Token::Ident };
    [+] => { $crate::parser::token::Token::Plus };
    [-] => { $crate::parser::token::Token::Minus };
    [*] => { $crate::parser::token::Token::Star };
    [/] => { $crate::parser::token::Token::Slash };
    [%] => { $crate::parser::token::Token::Percent };
    [^] => { $crate::parser::token::Token::Caret };
    [#] => { $crate::parser::token::Token::Hash };
    [&] => { $crate::parser::token::Token::Ampersand };
    [|] => { $crate::parser::token::Token::Pipe };
    [~] => { $crate::parser::token::Token::Tilde };
    [<<] => { $crate::parser::token::Token::DLAngle };
    [>>] => { $crate::parser::token::Token::DRAngle };
    [==] => { $crate::parser::token::Token::Eq };
    [~=] => { $crate::parser::token::Token::NotEq };
    [<=] => { $crate::parser::token::Token::LEq };
    [>=] => { $crate::parser::token::Token::GEq };
    [<] => { $crate::parser::token::Token::LAngle };
    [>] => { $crate::parser::token::Token::RAngle };
    [=] => { $crate::parser::token::Token::Assign };
    [D/] => { $crate::parser::token::Token::DSlash };
    [local] => { $crate::parser::token::Token::Local };
    [function] => { $crate::parser::token::Token::Function };
    [end] => { $crate::parser::token::Token::End };
    [in] => { $crate::parser::token::Token::In };
    [then] => { $crate::parser::token::Token::Then };
    [break] => { $crate::parser::token::Token::Break };
    [for] => { $crate::parser::token::Token::For };
    [do] => { $crate::parser::token::Token::Do };
    [until] => { $crate::parser::token::Token::Until };
    [else] => { $crate::parser::token::Token::Else };
    [while] => { $crate::parser::token::Token::While };
    [elseif] => { $crate::parser::token::Token::ElseIf };
    [if] => { $crate::parser::token::Token::If };
    [repeat] => { $crate::parser::token::Token::Repeat };
    [return] => { $crate::parser::token::Token::Return };
    [not] => { $crate::parser::token::Token::Not };
    [or] => { $crate::parser::token::Token::Or };
    [and] => { $crate::parser::token::Token::And };
    [const] => { $crate::parser::token::Token::Const };
    [close] => { $crate::parser::token::Token::Close };
    [nil] => { $crate::parser::token::Token::Nil };
    [true] => { $crate::parser::token::Token::True };
    [false] => { $crate::parser::token::Token::False };
    [string] => { $crate::parser::token::Token::String };
    [long_string] => { $crate::parser::token::Token::LongString };
    [int] => { $crate::parser::token::Token::Int };
    [hex_int] => { $crate::parser::token::Token::HexInt };
    [float] => { $crate::parser::token::Token::Float };
    [hex_float] => { $crate::parser::token::Token::HexFloat };
    ['('] => { $crate::parser::token::Token::LParen };
    [')'] => { $crate::parser::token::Token::RParen };
    ['{'] => { $crate::parser::token::Token::LCurly };
    ['}'] => { $crate::parser::token::Token::RCurly };
    ['['] => { $crate::parser::token::Token::LBracket };
    [']'] => { $crate::parser::token::Token::RBracket };
    [:] => { $crate::parser::token::Token::Colon };
    [::] => { $crate::parser::token::Token::DColon };
    [,] => { $crate::parser::token::Token::Comma };
    [.] => { $crate::parser::token::Token::Dot };
    [..] => { $crate::parser::token::Token::DDot };
    [...] => { $crate::parser::token::Token::TDot };
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![invalid] => "INVALID",
                T![eof] => "EOF",
                T![ident] => "IDENTIFIER",
                T![+] => "PLUS",
                T![-] => "MINUS",
                T![*] => "STAR",
                T![/] => "SLASH",
                T![%] => "PERCENT",
                T![^] => "CARET",
                T![#] => "HASH",
                T![&] => "AMPERSAND",
                T![|] => "PIPE",
                T![~] => "TILDE",
                T![<<] => "DLANGLE",
                T![>>] => "DRANGLE",
                T![==] => "EQ",
                T![~=] => "NOT_EQ",
                T![<=] => "LEQ",
                T![>=] => "GEQ",
                T![<] => "LANGLE",
                T![>] => "RANGLE",
                T![=] => "ASSIGN",
                T![D/] => "DSLASH",
                T![local] => "LOCAL",
                T![function] => "FUNCTION",
                T![end] => "END",
                T![in] => "IN",
                T![then] => "THEN",
                T![break] => "BREAK",
                T![for] => "FOR",
                T![do] => "DO",
                T![until] => "UNTIL",
                T![else] => "ELSE",
                T![while] => "WHILE",
                T![elseif] => "ELSEIF",
                T![if] => "IF",
                T![repeat] => "REPEAT",
                T![return] => "RETURN",
                T![not] => "NOT",
                T![or] => "OR",
                T![and] => "AND",
                T![const] => "CONST",
                T![close] => "CLOSE",
                T![nil] => "NIL",
                T![true] => "TRUE",
                T![false] => "FALSE",
                T![string] => "STRING",
                T![long_string] => "LONG_STRING",
                T![int] => "INT",
                T![hex_int] => "HEX_INT",
                T![float] => "FLOAT",
                T![hex_float] => "HEX_FLOAT",
                T!['('] => "RPAREN",
                T![')'] => "LPAREN",
                T!['{'] => "RCURLY",
                T!['}'] => "LCURLY",
                T!['['] => "RBRACKET",
                T![']'] => "LBRACKET",
                T![:] => "COLON",
                T![::] => "DCOLON",
                T![,] => "COMMA",
                T![.] => "DOT",
                T![..] => "DDOT",
                T![...] => "TDOT",
            }
        )
    }
}
