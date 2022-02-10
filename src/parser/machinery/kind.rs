use std::fmt::{self, Display};

use logos::{Lexer, Logos};

#[allow(clippy::manual_non_exhaustive)]
#[derive(Logos, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(u16)]
pub enum SyntaxKind {
    // Miscellaneous
    #[error]
    Invalid = 0,

    Tombstone,

    Eof,
    Root,
    BreakStmt,
    ReturnStmt,
    BlockStmt,
    WhileStmt,
    RepeatStmt,
    StmtList,
    IfStmt,
    ElseChain,
    ForNumStmt,
    ForGenStmt,
    FuncStmt,
    FuncArgs,
    SimpleExpr,
    Expr,
    VarArgExpr,
    BinOp,
    FuncCall,
    Index,
    ExprList,
    DeclStmt,
    DeclTarget,
    FuncExpr,
    PrefixOp,
    TableExpr,
    TableArrayElem,
    TableMapElem,
    TableGenericElem,
    AssignStmt,
    LiteralExpr,

    #[regex(r"[ \n\t\f\r]+", logos::skip)]
    Whitespace,

    #[regex("--", skip_comment)]
    Comment,

    // Operators
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
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 3)]
    Ident,

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

    #[token(";")]
    Semicolon,

    #[doc(hidden)]
    #[allow(clippy::upper_case_acronyms)]
    __LAST,
}

impl SyntaxKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, SyntaxKind::Whitespace | SyntaxKind::Comment)
    }
}

fn long_string(lexer: &mut Lexer<SyntaxKind>) {
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

fn skip_comment(lexer: &mut Lexer<SyntaxKind>) -> logos::Skip {
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

fn skip_long_comment(lexer: &mut Lexer<SyntaxKind>, delim_len: usize) {
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
    if !slice.starts_with("[[") && !slice.starts_with("[=]") {
        return None;
    }

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
    [invalid] => { $crate::parser::machinery::kind::SyntaxKind::Invalid };
    [tombstone] => { $crate::parser::machinery::kind::SyntaxKind::Tombstone };
    [eof] => { $crate::parser::machinery::kind::SyntaxKind::Eof };
    [root] => { $crate::parser::machinery::kind::SyntaxKind::Root };
    [break_stmt] => { $crate::parser::machinery::kind::SyntaxKind::BreakStmt };
    [return_stmt] => { $crate::parser::machinery::kind::SyntaxKind::ReturnStmt };
    [block_stmt] => { $crate::parser::machinery::kind::SyntaxKind::BlockStmt };
    [while_stmt] => { $crate::parser::machinery::kind::SyntaxKind::WhileStmt };
    [repeat_stmt] => { $crate::parser::machinery::kind::SyntaxKind::RepeatStmt };
    [stmt_list] => { $crate::parser::machinery::kind::SyntaxKind::StmtList };
    [if_stmt] => { $crate::parser::machinery::kind::SyntaxKind::IfStmt };
    [else_chain] => { $crate::parser::machinery::kind::SyntaxKind::ElseChain };
    [for_num_stmt] => { $crate::parser::machinery::kind::SyntaxKind::ForNumStmt };
    [for_gen_stmt] => { $crate::parser::machinery::kind::SyntaxKind::ForGenStmt };
    [func_stmt] => { $crate::parser::machinery::kind::SyntaxKind::FuncStmt };
    [func_args] => { $crate::parser::machinery::kind::SyntaxKind::FuncArgs };
    [simple_expr] => { $crate::parser::machinery::kind::SyntaxKind::SimpleExpr };
    [expr] => { $crate::parser::machinery::kind::SyntaxKind::Expr };
    [vararg_expr] => { $crate::parser::machinery::kind::SyntaxKind::VarArgExpr };
    [bin_op] => { $crate::parser::machinery::kind::SyntaxKind::BinOp };
    [func_call] => { $crate::parser::machinery::kind::SyntaxKind::FuncCall };
    [index] => { $crate::parser::machinery::kind::SyntaxKind::Index };
    [expr_list] => { $crate::parser::machinery::kind::SyntaxKind::ExprList };
    [decl_stmt] => { $crate::parser::machinery::kind::SyntaxKind::DeclStmt };
    [decl_target] => { $crate::parser::machinery::kind::SyntaxKind::DeclTarget };
    [func_expr] => { $crate::parser::machinery::kind::SyntaxKind::FuncExpr };
    [prefix_op] => { $crate::parser::machinery::kind::SyntaxKind::PrefixOp };
    [table_expr] => { $crate::parser::machinery::kind::SyntaxKind::TableExpr };
    [table_array_elem] => { $crate::parser::machinery::kind::SyntaxKind::TableArrayElem };
    [table_map_elem] => { $crate::parser::machinery::kind::SyntaxKind::TableMapElem };
    [table_generic_elem] => { $crate::parser::machinery::kind::SyntaxKind::TableGenericElem };
    [assign_stmt] => { $crate::parser::machinery::kind::SyntaxKind::AssignStmt };
    [literal_expr] => { $crate::parser::machinery::kind::SyntaxKind::LiteralExpr };
    [ident] => { $crate::parser::machinery::kind::SyntaxKind::Ident };
    [+] => { $crate::parser::machinery::kind::SyntaxKind::Plus };
    [-] => { $crate::parser::machinery::kind::SyntaxKind::Minus };
    [*] => { $crate::parser::machinery::kind::SyntaxKind::Star };
    [/] => { $crate::parser::machinery::kind::SyntaxKind::Slash };
    [%] => { $crate::parser::machinery::kind::SyntaxKind::Percent };
    [^] => { $crate::parser::machinery::kind::SyntaxKind::Caret };
    [#] => { $crate::parser::machinery::kind::SyntaxKind::Hash };
    [&] => { $crate::parser::machinery::kind::SyntaxKind::Ampersand };
    [|] => { $crate::parser::machinery::kind::SyntaxKind::Pipe };
    [~] => { $crate::parser::machinery::kind::SyntaxKind::Tilde };
    [<<] => { $crate::parser::machinery::kind::SyntaxKind::DLAngle };
    [>>] => { $crate::parser::machinery::kind::SyntaxKind::DRAngle };
    [==] => { $crate::parser::machinery::kind::SyntaxKind::Eq };
    [~=] => { $crate::parser::machinery::kind::SyntaxKind::NotEq };
    [<=] => { $crate::parser::machinery::kind::SyntaxKind::LEq };
    [>=] => { $crate::parser::machinery::kind::SyntaxKind::GEq };
    [<] => { $crate::parser::machinery::kind::SyntaxKind::LAngle };
    [>] => { $crate::parser::machinery::kind::SyntaxKind::RAngle };
    [=] => { $crate::parser::machinery::kind::SyntaxKind::Assign };
    [D/] => { $crate::parser::machinery::kind::SyntaxKind::DSlash };
    [local] => { $crate::parser::machinery::kind::SyntaxKind::Local };
    [function] => { $crate::parser::machinery::kind::SyntaxKind::Function };
    [end] => { $crate::parser::machinery::kind::SyntaxKind::End };
    [in] => { $crate::parser::machinery::kind::SyntaxKind::In };
    [then] => { $crate::parser::machinery::kind::SyntaxKind::Then };
    [break] => { $crate::parser::machinery::kind::SyntaxKind::Break };
    [for] => { $crate::parser::machinery::kind::SyntaxKind::For };
    [do] => { $crate::parser::machinery::kind::SyntaxKind::Do };
    [until] => { $crate::parser::machinery::kind::SyntaxKind::Until };
    [else] => { $crate::parser::machinery::kind::SyntaxKind::Else };
    [while] => { $crate::parser::machinery::kind::SyntaxKind::While };
    [elseif] => { $crate::parser::machinery::kind::SyntaxKind::ElseIf };
    [if] => { $crate::parser::machinery::kind::SyntaxKind::If };
    [repeat] => { $crate::parser::machinery::kind::SyntaxKind::Repeat };
    [return] => { $crate::parser::machinery::kind::SyntaxKind::Return };
    [not] => { $crate::parser::machinery::kind::SyntaxKind::Not };
    [or] => { $crate::parser::machinery::kind::SyntaxKind::Or };
    [and] => { $crate::parser::machinery::kind::SyntaxKind::And };
    [const] => { $crate::parser::machinery::kind::SyntaxKind::Const };
    [close] => { $crate::parser::machinery::kind::SyntaxKind::Close };
    [nil] => { $crate::parser::machinery::kind::SyntaxKind::Nil };
    [true] => { $crate::parser::machinery::kind::SyntaxKind::True };
    [false] => { $crate::parser::machinery::kind::SyntaxKind::False };
    [string] => { $crate::parser::machinery::kind::SyntaxKind::String };
    [long_string] => { $crate::parser::machinery::kind::SyntaxKind::LongString };
    [int] => { $crate::parser::machinery::kind::SyntaxKind::Int };
    [hex_int] => { $crate::parser::machinery::kind::SyntaxKind::HexInt };
    [float] => { $crate::parser::machinery::kind::SyntaxKind::Float };
    [hex_float] => { $crate::parser::machinery::kind::SyntaxKind::HexFloat };
    ['('] => { $crate::parser::machinery::kind::SyntaxKind::LParen };
    [')'] => { $crate::parser::machinery::kind::SyntaxKind::RParen };
    ['{'] => { $crate::parser::machinery::kind::SyntaxKind::LCurly };
    ['}'] => { $crate::parser::machinery::kind::SyntaxKind::RCurly };
    ['['] => { $crate::parser::machinery::kind::SyntaxKind::LBracket };
    [']'] => { $crate::parser::machinery::kind::SyntaxKind::RBracket };
    [:] => { $crate::parser::machinery::kind::SyntaxKind::Colon };
    [::] => { $crate::parser::machinery::kind::SyntaxKind::DColon };
    [,] => { $crate::parser::machinery::kind::SyntaxKind::Comma };
    [.] => { $crate::parser::machinery::kind::SyntaxKind::Dot };
    [..] => { $crate::parser::machinery::kind::SyntaxKind::DDot };
    [...] => { $crate::parser::machinery::kind::SyntaxKind::TDot };
    [;] => { $crate::parser::machinery::kind::SyntaxKind::Semicolon };
    [__LAST] => { $crate::parser::machinery::kind::SyntaxKind::__LAST };
}

impl Display for SyntaxKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T![invalid] => "INVALID",
                T![eof] => "EOF",
                T![root] => "ROOT",
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
                _ => unreachable!(),
            }
        )
    }
}
