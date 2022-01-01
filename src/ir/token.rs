use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[error]
    Invalid,

    #[regex(r"[a-zA-Z][a-zA-Z0-9]*")]
    Identifier,

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

    #[token("goto")]
    Goto,

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

    #[token("nil")]
    Nil,

    #[token("false")]
    False,

    #[token("true")]
    True,

    #[regex(r#""((\\"|\\\\)|[^\\"])*""#)]
    String,

    #[regex(r"\[=*\[", |x| long_lexeme(x, 0))]
    LongString,

    #[regex(r"[0-9]+")]
    Integer,

    #[regex(r"0x[0-9a-fA-F]+")]
    HexInteger,

    #[regex(r"[0-9]*\.[0-9]+([eE][+-][0-9]+)?")]
    Float,

    #[regex(r"0x[0-9a-fA-F]*\.[0-9a-fA-F]+([pP][+-][0-9a-fA-F]+)?")]
    HexFloat,
    
    #[token("not")]
    Not,

    #[token("~")]
    Tilde,

    #[token("or")]
    Or,

    #[token("and")]
    And,

    #[token("+")]
    Plus,

    #[token("-")]
    Dash,

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

    #[token("<<")]
    LeftLeft,

    #[token(">>")]
    RightRight,

    #[token("//")]
    SlashSlash,

    #[token("==")]
    EqualEqual,

    #[token("~=")]
    TildeEqual,

    #[token("<=")]
    LeftEqual,

    #[token(">=")]
    RightEqual,

    #[token("<")]
    Left,

    #[token(">")]
    Right,

    #[token("=")]
    Equal,

    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftCurly,

    #[token("}")]
    RightCurly,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[token(";")]
    Semicolon,

    #[token(",")]
    Comma,

    #[token(".")]
    Dot,

    #[token("..")]
    DoubleDot,

    #[token("...")]
    TripleDot,

    #[token("--")]
    DashDash,

    #[regex(r"--\[=*\[", |x| long_lexeme(x, 2))]
    LongComment,

    #[token("const")]
    Const,
 
    #[token("close")]
    Close,
}

fn long_lexeme(lex: &mut Lexer<Token>, skips: usize) -> bool {
    let count = lex.slice().len() - skips;
    let rem = lex.remainder();
 
    for (i, _) in rem.char_indices() {
        match rem.get(i..i + count) {
            Some(slice) => {
                if is_long_delimiter(slice, ']') {
                    lex.bump(i + 1);
                    return true;
                }
            }
 
            None => break,
        }
    }
 
    false
}
 
fn is_long_delimiter(slice: &str, delim: char) -> bool {
    if !slice.starts_with(delim) || !slice.ends_with(delim) {
        return false;
    }
 
    slice.chars().filter(|c| *c == '=').count() == slice.len() - 2
}
