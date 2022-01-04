mod binding_power;
mod state;
mod token;

use binding_power::{
    infix_binding_power,
    prefix_binding_power,
    CALL_BINDING_POWER,
    INDEX_BINDING_POWER,
};
use either::Either;
use hexf_parse::parse_hexf64;
use state::State;
use token::Token;

use super::syntax_tree::{
    Assign,
    AssignTarget,
    BinaryExpr,
    BinaryOp,
    Do,
    Expr,
    ForGeneric,
    ForNumeric,
    Function,
    FunctionCall,
    Ident,
    If,
    IfChain,
    Label,
    Literal,
    NumLiteral,
    Repeat,
    Return,
    SimpleExpr,
    Stmt,
    SyntaxTree,
    Table,
    TableElement,
    UnaryExpr,
    UnaryOp,
    While,
};
use crate::T;

pub fn parse(source: &str) -> (SyntaxTree, Vec<ariadne::Report>) {
    let mut state = State::new(source);
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![eof] => break,
            _ => {
                let stmt = parse_stmt(&mut state);
                block.push(stmt);
            },
        }
    }

    (SyntaxTree { block }, state.result())
}

fn parse_block(state: &mut State) -> Vec<Stmt> {
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![eof] => break,
            T![end] => {
                state.eat(T![end]);
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt);
            },
        }
    }

    block
}

fn parse_stmt(state: &mut State) -> Stmt {
    loop {
        match state.peek() {
            T![::] => {
                let item = parse_label(state);
                return Stmt::Label(item);
            },
            T![do] => {
                let item = parse_do(state);
                return Stmt::Do(item);
            },
            T![while] => {
                let item = parse_while(state);
                return Stmt::While(item);
            },
            T![repeat] => {
                let item = parse_repeat(state);
                return Stmt::Repeat(item);
            },
            T![if] => {
                let item = parse_if(state);
                return Stmt::If(item);
            },
            T![for] => match parse_for(state) {
                Either::Left(numeric) => return Stmt::ForNumeric(numeric),
                Either::Right(generic) => return Stmt::ForGeneric(generic),
            },
            T![return] => {
                let item = parse_return(state);
                return Stmt::Return(item);
            },
            T![break] => {
                state.next();
                return Stmt::Break;
            },
            T![endstmt] => {
                state.next();
                continue;
            },
            T![function] => {
                let item = parse_named_function(state, false);
                return Stmt::Assign(item);
            },
            T![local] => {
                state.next();

                if state.at(T![function]) {
                    let item = parse_named_function(state, true);
                    return Stmt::Assign(item);
                }

                let item = parse_simple_expr(state);
                let is_const = if state.at(T![const]) {
                    state.next();
                    true
                } else {
                    false
                };

                let target = AssignTarget {
                    target: item,
                    is_const,
                };

                return Stmt::Assign(parse_assign_trail(state, true, target));
            },
            _ => {
                let item = parse_simple_expr(state);
                let is_const = if state.at(T![const]) {
                    state.next();
                    true
                } else {
                    false
                };

                if matches!(state.peek(), T![=] | T![,]) {
                    let target = AssignTarget {
                        target: item,
                        is_const,
                    };

                    return Stmt::Assign(parse_assign_trail(state, false, target));
                }

                return Stmt::SimpleExpr(item);
            },
        }
    }
}

fn parse_assign_trail(state: &mut State, is_local: bool, first_target: AssignTarget) -> Assign {
    let mut targets = vec![first_target];

    loop {
        match state.peek() {
            T![,] => {
                state.next();
                let item = parse_simple_expr(state);
                let is_const = if state.at(T![const]) {
                    state.next();
                    true
                } else {
                    false
                };

                let target = AssignTarget {
                    target: item,
                    is_const,
                };

                targets.push(target);
            },
            _ => break,
        }
    }

    state.eat(T![=]);
    let first_value = parse_expr(state);
    let mut values = vec![first_value];

    loop {
        match state.peek() {
            T![,] => {
                state.next();
                let value = parse_expr(state);
                values.push(value);
            },
            _ => break,
        }
    }

    Assign {
        is_local,
        target: targets,
        value: values,
    }
}

fn parse_simple_expr(state: &mut State) -> SimpleExpr {
    let first = parse_ident(state);
    let mut expr = SimpleExpr::Ident(first);

    loop {
        match state.peek() {
            T![.] => {
                state.next();
                let ident = parse_ident(state);
                expr = SimpleExpr::Property(Box::new(expr), Expr::Ident(ident));
            },
            T!['('] => {
                expr = SimpleExpr::FunctionCall(FunctionCall {
                    func: expr.into(),
                    args: parse_function_call(state),
                });
            },
            T![:] => {
                state.next();
                let ident = parse_ident(state);
                expr = SimpleExpr::Method(Box::new(expr), ident);
            },
            T!['['] => {
                state.next();
                let index = parse_expr(state);
                state.eat(T![']']);
                expr = SimpleExpr::Property(Box::new(expr), index);
            },
            _ => break,
        }
    }

    expr
}

fn parse_expr(state: &mut State) -> Expr {
    expr_bp(state, 0)
}

fn expr_bp(state: &mut State, min_bp: i32) -> Expr {
    let mut lhs = expr_bp_lhs(state);

    loop {
        let t = match state.peek() {
            T![eof] => break,
            T![function] => {
                let item = parse_anon_function(state);
                return Expr::Function(item);
            },
            t if token_is_literal(t) => {
                let item = parse_literal(state);
                return Expr::Literal(item);
            },
            T!['{'] => {
                let item = parse_table(state);
                return Expr::Table(item);
            },
            t if token_is_other_op(t) => t,
            t => panic!("invalid token {:?}", t),
        };

        if t == T!['('] && CALL_BINDING_POWER >= min_bp {
            let args = parse_function_call(state);
            lhs = Expr::FunctionCall(Box::new(FunctionCall { func: lhs, args }));
            continue;
        }

        if t == T!['['] && INDEX_BINDING_POWER >= min_bp {
            state.next();
            let rhs = expr_bp(state, 0);
            state.eat(T![']']);

            lhs = Expr::Binary(Box::new(BinaryExpr {
                op: BinaryOp::Property,
                lhs,
                rhs,
            }));

            continue;
        }

        if let Some((l_bp, r_bp)) = infix_binding_power(t) {
            if l_bp < min_bp {
                break;
            }

            state.next();
            let rhs = expr_bp(state, r_bp);
            lhs = Expr::Binary(Box::new(BinaryExpr {
                op: token_to_binary_op(t),
                lhs,
                rhs,
            }));

            continue;
        }

        break;
    }

    lhs
}

fn expr_bp_lhs(state: &mut State) -> Expr {
    let t = state.next();
    if T![ident] == t {
        return Expr::Ident(parse_ident(state));
    }

    if T!['('] == t {
        let lhs = expr_bp(state, 0);
        state.eat(T![')']);
        return lhs;
    }

    if let Some(op) = token_to_unary_op(t) {
        let ((), r_bp) = prefix_binding_power(t);
        let rhs = expr_bp(state, r_bp);
        return Expr::Unary(Box::new(UnaryExpr { op, expr: rhs }));
    }

    panic!("invalid token {:?}", t);
}

fn parse_label(state: &mut State) -> Label {
    state.eat(T![::]);
    let name = parse_ident(state);
    state.eat(T![::]);
    Label { ident: name }
}

fn parse_do(state: &mut State) -> Do {
    state.eat(T![do]);
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![end] => {
                state.eat(T![end]);
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt)
            },
        }
    }

    Do { block }
}

fn parse_while(state: &mut State) -> While {
    state.next();
    let condition = parse_expr(state);
    state.eat(T![do]);
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![end] => {
                state.eat(T![end]);
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt)
            },
        }
    }

    While { condition, block }
}

fn parse_repeat(state: &mut State) -> Repeat {
    state.next();
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![until] => {
                state.eat(T![until]);
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt)
            },
        }
    }

    let condition = parse_expr(state);
    Repeat { condition, block }
}

fn parse_if(state: &mut State) -> If {
    state.next();
    let condition = parse_expr(state);
    state.eat(T![then]);
    let mut chain = None;
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![else] => {
                state.next();
                let else_block = parse_block(state);
                chain = Some(Box::new(IfChain::Else(else_block)))
            },
            T![elseif] => {
                let elseif = parse_if(state);
                chain = Some(Box::new(IfChain::ElseIf(elseif)));
            },
            T![end] => {
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt)
            },
        }
    }

    If {
        condition,
        block,
        or: chain,
    }
}

fn parse_for(state: &mut State) -> Either<ForNumeric, ForGeneric> {
    state.eat(T![for]);
    let first_var = parse_ident(state);

    if state.at(T![=]) {
        let item = parse_for_numeric(state, first_var);
        return Either::Left(item);
    } else {
        let item = parse_for_generic(state, first_var);
        return Either::Right(item);
    }
}

fn parse_for_numeric(state: &mut State, first_var: Ident) -> ForNumeric {
    state.eat(T![=]);
    let start = parse_expr(state);
    state.eat(T![,]);
    let end = parse_expr(state);

    let step = if state.at(T![,]) {
        state.next();
        Some(parse_expr(state))
    } else {
        None
    };

    state.eat(T![do]);
    let block = parse_block(state);

    ForNumeric {
        variable: first_var,
        start,
        end,
        step,
        block,
    }
}

fn parse_for_generic(state: &mut State, first_var: Ident) -> ForGeneric {
    let mut args = vec![first_var];

    loop {
        match state.peek() {
            T![,] => {
                state.next();
                args.push(parse_ident(state));
            },
            T![endstmt] => todo!(),
            T![in] => {
                state.next();
                break;
            },
            t => panic!("invalid token {:?}", t),
        }
    }

    let yielder = parse_expr(state);
    state.eat(T![do]);
    let block = parse_block(state);

    ForGeneric {
        targets: args,
        yielder,
        block,
    }
}

fn parse_return(state: &mut State) -> Return {
    let mut values = Vec::new();

    loop {
        match state.peek() {
            T![endstmt] => {
                state.next();
                break;
            },
            T![,] => continue,
            _ => {
                let arg = parse_expr(state);
                values.push(arg);
            },
        }
    }

    Return { values }
}

fn parse_ident(state: &mut State) -> Ident {
    state.eat(T![ident]);
    Ident {
        name: state.slice().to_string(),
    }
}

fn parse_named_function(state: &mut State, is_local: bool) -> Assign {
    state.eat(T![function]);
    let name = parse_ident(state);
    let function = parse_function_trail(state);
    let target = AssignTarget {
        target: SimpleExpr::Ident(name),
        is_const: false,
    };

    Assign {
        is_local,
        target: vec![target],
        value: vec![Expr::Function(function)],
    }
}

fn parse_anon_function(state: &mut State) -> Function {
    state.eat(T![function]);
    parse_function_trail(state)
}

fn parse_function_trail(state: &mut State) -> Function {
    state.eat(T!['(']);
    let mut args = Vec::new();
    let mut block = Vec::new();

    loop {
        match state.peek() {
            T![ident] => {
                let arg = parse_ident(state);
                args.push(arg);
            },
            T![')'] => {
                state.next();
                break;
            },
            T![,] => continue,
            t => panic!("invalid token {:?}", t),
        }
    }

    loop {
        match state.peek() {
            T![end] => {
                state.next();
                break;
            },
            _ => {
                let stmt = parse_stmt(state);
                block.push(stmt)
            },
        }
    }

    Function { args, block }
}

fn parse_literal(state: &mut State) -> Literal {
    match state.peek() {
        T![nil] => {
            state.next();
            Literal::Nil
        },
        T![true] => {
            state.next();
            Literal::Boolean(true)
        },
        T![false] => {
            state.next();
            Literal::Boolean(false)
        },
        T![string] => Literal::String(parse_string(state)),
        T![long_string] => Literal::String(parse_long_string(state)),
        T![int] => Literal::Num(NumLiteral::Int(parse_int(state))),
        T![hex_int] => Literal::Num(NumLiteral::Int(parse_hex_int(state))),
        T![float] => Literal::Num(NumLiteral::Float(parse_float(state))),
        T![hex_float] => Literal::Num(NumLiteral::Float(parse_hex_float(state))),
        t => panic!("invalid token {:?}", t),
    }
}

fn parse_string(state: &mut State) -> String {
    state.eat(T![string]);
    let mut value = String::new();
    let mut chars = state.slice().chars();
    let mut escaped = false;
    let delim = chars.next().unwrap();

    for char in chars {
        match char {
            _ if escaped => {
                value.push(char);
                escaped = false;
            },
            '\\' => escaped = true,
            _ if char == delim => break,
            _ => value.push(char),
        }
    }

    value
}

fn parse_long_string(state: &mut State) -> String {
    let mut chars = state.slice().chars();
    chars.next();
    let delim_len = chars.by_ref().take_while(|c| *c != '[').count() + 1;
    chars.take(state.slice().len() - delim_len * 2).collect()
}

fn parse_int(state: &mut State) -> i64 {
    state.next();
    state.slice().parse().unwrap()
}

fn parse_hex_int(state: &mut State) -> i64 {
    state.next();
    let raw = &state.slice()[2..];
    i64::from_str_radix(raw, 16).unwrap()
}

fn parse_float(state: &mut State) -> f64 {
    state.next();
    state.slice().parse().unwrap()
}

fn parse_hex_float(state: &mut State) -> f64 {
    state.next();

    if let Ok(value) = parse_hexf64(state.slice(), true) {
        value
    } else {
        let span = state.span();
        let report = ariadne::Report::build(ariadne::ReportKind::Error, (), span.start)
            .with_message("Invalid hexadecimal float literal")
            .with_label(ariadne::Label::new(span).with_message("Invalid literal found here"))
            .finish();

        state.report(report);
        return 0.0;
    }
}

fn parse_function_call(state: &mut State) -> Vec<Expr> {
    state.next();
    let mut args = Vec::new();

    loop {
        match state.peek() {
            T![')'] => {
                state.next();
                break;
            },
            T![,] => continue,
            _ => {
                let arg = parse_expr(state);
                args.push(arg);
            },
        }
    }

    args
}

fn parse_table(state: &mut State) -> Table {
    state.next();
    let mut elements = Vec::new();

    loop {
        match state.peek() {
            T!['}'] => {
                state.next();
                break;
            },
            T!['['] => elements.push(parse_table_element_expr(state)),
            T![,] => continue,
            _ => {
                let first = parse_expr(state);

                if matches!(first, Expr::Ident(_)) && state.at(T![=]) {
                    state.next();
                    let value = parse_expr(state);
                    elements.push(TableElement {
                        key: Some(first),
                        value,
                    });
                } else {
                    elements.push(TableElement {
                        key: None,
                        value: first,
                    });
                }
            },
        }
    }

    Table { elements }
}

fn parse_table_element_expr(state: &mut State) -> TableElement {
    state.next();
    let key = parse_expr(state);
    state.eat(T![']']);
    state.eat(T![=]);
    let value = parse_expr(state);

    TableElement {
        key: Some(key),
        value,
    }
}

fn token_is_literal(token: Token) -> bool {
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

fn token_to_unary_op(token: Token) -> Option<UnaryOp> {
    match token {
        T![not] => Some(UnaryOp::Not),
        T![#] => Some(UnaryOp::Len),
        T![+] => Some(UnaryOp::Pos),
        T![-] => Some(UnaryOp::Neg),
        T![~] => Some(UnaryOp::BitNot),
        _ => None,
    }
}

fn token_to_binary_op(token: Token) -> BinaryOp {
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

fn token_is_other_op(token: Token) -> bool {
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

// TODO: handle newline and semicolon and eof
// TODO: use peek instead of next?
// TODO: eat/next?
// TODO: force comma in lists
// TODO: support various string escape sequences

#[cfg(test)]
mod tests {
    use std::fs::{read_dir, read_to_string};

    use super::{super::syntax_tree::SyntaxTree, parse};

    #[test]
    fn parse_check_tests() {
        for entry in read_dir("test-files/check").unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let source = read_to_string(path).unwrap();
            let (_syntax_tree, reports) = parse(&source);
            assert!(reports.is_empty());
        }
    }

    #[test]
    fn parse_and_verify_simple_calc() {
        let source = read_to_string("test-files/simple/calc.lua").unwrap();
        let (syntax_tree, reports) = parse(&source);
        assert!(reports.is_empty());
        let expected = SyntaxTree { block: Vec::new() };
        assert_eq!(expected, syntax_tree);
    }
}
