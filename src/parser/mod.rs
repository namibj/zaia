mod binding_power;
mod state;
mod token;

use either::Either;
use hexf_parse::parse_hexf64;
use state::State;
use token::Token;

use super::syntax_tree::{
    Assign,
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
    Stmt,
    SyntaxTree,
    Table,
    UnaryExpr,
    UnaryOp,
    While,
};
use crate::T;

pub fn parse(source: &str) -> (SyntaxTree, Vec<ariadne::Report>) {
    let mut state = State::new(source);
    let mut tree = SyntaxTree { block: Vec::new() };

    loop {
        match state.peek() {
            T![eof] => break,
            _ => {
                let stmt = parse_stmt(&mut state);
                tree.block.push(stmt);
            },
        }
    }

    (tree, state.result())
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
                parse_break(state);
                return Stmt::Break;
            },
            _ => {
                let item = parse_expr(state);
                return Stmt::Expr(item);
            },
        }
    }
}

fn parse_expr(state: &mut State) -> Expr {
    match state.peek() {
        T![ident] => {
            let item = parse_ident(state);
            return Expr::Variable(item);
        },
        // TODO: handle ops
        T![function] => match parse_function(state) {
            Either::Left(assign) => return Expr::Assign(Box::new(assign)),
            Either::Right(function) => return Expr::Function(function),
        },
        t if token_is_literal(t) => {
            let item = parse_literal(state);
            return Expr::Literal(item);
        },
        T!['{'] => {
            let item = parse_table(state);
            return Expr::Table(item);
        },
        // TODO: handle function calls
        // TODO: handle assign
        _ => todo!(),
    }
}

fn parse_label(state: &mut State) -> Label {
    state.eat(T![::]);
    let name = parse_ident(state);
    state.eat(T![::]);
    Label { ident: name }
}

fn parse_do(state: &mut State) -> Do {
    todo!()
}

fn parse_while(state: &mut State) -> While {
    todo!()
}

fn parse_repeat(state: &mut State) -> Repeat {
    todo!()
}

fn parse_if(state: &mut State) -> If {
    todo!()
}

fn parse_for(state: &mut State) -> Either<ForNumeric, ForGeneric> {
    todo!()
}

fn parse_for_numeric(state: &mut State) -> ForNumeric {
    todo!()
}

fn parse_for_generic(state: &mut State) -> ForGeneric {
    todo!()
}

fn parse_return(state: &mut State) -> Return {
    todo!()
}

fn parse_break(state: &mut State) {
    todo!()
}

fn parse_ident(state: &mut State) -> Ident {
    todo!()
}

fn parse_unary_expr(state: &mut State) -> UnaryExpr {
    todo!()
}

fn parse_binary_expr(state: &mut State) -> BinaryExpr {
    todo!()
}

fn parse_function(state: &mut State) -> Either<Assign, Function> {
    todo!()
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
        _ => unreachable!(),
    }
}

// TODO: Support strings using single quotes.
// TODO: Support various escape sequences.
fn parse_string(state: &mut State) -> String {
    state.eat(T![string]);
    let mut value = String::new();
    let mut chars = state.slice().chars();
    let mut escaped = false;
    chars.next();

    for char in state.slice().chars() {
        match char {
            '"' if !escaped => break,
            '\\' if !escaped => escaped = true,
            '\\' if escaped => {
                escaped = false;
                value.push(char);
            },
            _ => {
                escaped = false;
                value.push(char);
            },
        }
    }

    value
}

// TODO: Support long strings.
fn parse_long_string(state: &mut State) -> String {
    todo!()
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

fn parse_function_call(state: &mut State) -> FunctionCall {
    todo!()
}

fn parse_table(state: &mut State) -> Table {
    todo!()
}

fn parse_assign(state: &mut State) -> Assign {
    todo!()
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
