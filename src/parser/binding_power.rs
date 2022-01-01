use crate::T;
use super::token::Token;

fn infix_binding_power(op: Token) -> (u8, u8) {
    match op {
        T![+] | T![-] => (1, 2),
        T![*] | T![/] | T![D/] => (3, 4),
        _ => panic!("bad infix op: {}", op),
    }
}
