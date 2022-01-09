use super::token::Token;
use crate::{
    syntax_tree::{BinaryOp, UnaryOp},
    T,
};

pub fn token_is_literal(token: Token) -> bool {
    matches!(
        token,
        T![nil]
            | T![false]
            | T![true]
            | T![int]
            | T![hex_int]
            | T![float]
            | T![hex_float]
            | T![string]
            | T![long_string]
    )
}

pub fn token_is_other_op(token: Token) -> bool {
    matches!(
        token,
        T![or]
            | T![and]
            | T![+]
            | T![-]
            | T![*]
            | T![/]
            | T![D/]
            | T![^]
            | T![%]
            | T![&]
            | T![|]
            | T![<<]
            | T![>>]
            | T![==]
            | T![~]
            | T![~=]
            | T![<=]
            | T![>=]
            | T![<]
            | T![>]
            | T![:]
            | T![.]
            | T![..]
            | T!['[']
    )
}

pub fn token_is_expr_start(token: Token) -> bool {
    token == T![ident]
        || token == T!['(']
        || token_is_literal(token)
        || token_to_unary_op(token).is_some()
}

pub fn token_to_unary_op(token: Token) -> Option<UnaryOp> {
    match token {
        T![not] => Some(UnaryOp::Not),
        T![#] => Some(UnaryOp::Len),
        T![+] => Some(UnaryOp::Pos),
        T![-] => Some(UnaryOp::Neg),
        T![~] => Some(UnaryOp::BitNot),
        _ => None,
    }
}

pub fn token_to_binary_op(token: Token) -> BinaryOp {
    match token {
        T![or] => BinaryOp::Or,
        T![and] => BinaryOp::And,
        T![+] => BinaryOp::Add,
        T![-] => BinaryOp::Sub,
        T![*] => BinaryOp::Mul,
        T![/] => BinaryOp::Div,
        T![D/] => BinaryOp::FloorDiv,
        T![^] => BinaryOp::Exp,
        T![%] => BinaryOp::Mod,
        T![&] => BinaryOp::BitAnd,
        T![|] => BinaryOp::BitOr,
        T![<<] => BinaryOp::LeftShift,
        T![>>] => BinaryOp::RightShift,
        T![==] => BinaryOp::Equals,
        T![~] => BinaryOp::Xor,
        T![~=] => BinaryOp::NotEquals,
        T![<=] => BinaryOp::LesserEquals,
        T![>=] => BinaryOp::GreaterEquals,
        T![<] => BinaryOp::Greater,
        T![>] => BinaryOp::Lesser,
        T![.] => BinaryOp::Property,
        T![:] => BinaryOp::Method,
        T![..] => BinaryOp::Concat,
        t => panic!("invalid token {:?}", t),
    }
}
