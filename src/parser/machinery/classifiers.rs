use super::kind::SyntaxKind;
use crate::T;

pub fn token_is_literal(token: SyntaxKind) -> bool {
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

pub fn token_is_expr_start(token: SyntaxKind) -> bool {
    token == T![ident]
        || token == T!['(']
        || token_is_literal(token)
        || token_is_unary_op(token)
        || token == T!['{']
        || token == T![function]
        || token == T![...]
}

pub fn token_is_unary_op(token: SyntaxKind) -> bool {
    matches!(token, T![not] | T![+] | T![-] | T![#] | T![~])
}

pub fn token_is_binary_op(token: SyntaxKind) -> bool {
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
            | T![>]
            | T![<]
            | T![.]
            | T![:]
            | T![..]
    )
}
