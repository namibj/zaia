use super::token::Token;
use crate::T;

pub fn prefix_binding_power(op: Token) -> ((), u8) { 
    match op {
        T![not] => ((), 6),
        T![+] | T![-] | T![~] | T![#] => ((), 7),
        _ => panic!("bad prefix op: {:?}", op),
    }
}

pub fn infix_binding_power(op: Token) -> (u8, u8) {
    match op {
        T![..] => (1, 2),
        T![+] | T![-] => (3, 4),
        T![*] | T![/] | T![D/] => (5, 6),
        T![.] => (8, 7),
        _ => panic!("bad infix op: {}", op),
    }
}
