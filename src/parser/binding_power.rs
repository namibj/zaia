use super::token::Token;
use crate::T;

pub fn prefix_binding_power(op: Token) -> ((), u8) {
    match op {
        T![not] | T![+] | T![-] | T![#] | T![~] => ((), 21),
        _ => panic!("bad prefix op: {}", op),
    }
}

pub fn infix_binding_power(op: Token) -> Option<(u8, u8)> {
    Some(match op {
        T![or] => (1, 2),
        T![and] => (3, 4),
        T![<] | T![>] | T![<=] | T![>=] | T![~=] | T![==] => (5, 6),
        T![|] => (7, 8),
        T![~] => (9, 10),
        T![&] => (11, 12),
        T![<<] | T![>>] => (13, 14),
        T![..] => (16, 15),
        T![+] | T![-] => (17, 18),
        T![*] | T![/] | T![D/] | T![%] => (19, 20),
        T![^] => (22, 21),
        _ => return None,
    })
}
