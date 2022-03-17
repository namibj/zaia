use std::{convert::Infallible, ops};

use super::{
    super::{value::Value, Error},
    ctx::Ctx,
};
use crate::parser::syntax::{
    Assign,
    Break,
    Decl,
    Do,
    Expr,
    ForGen,
    ForNum,
    Func,
    If,
    Repeat,
    Return,
    Root,
    Stmt,
    While,
};

pub trait Eval {
    fn eval(&self, ctx: &mut Ctx) -> Result;
}

pub enum Result<T = Value> {
    Value(T),
    Return(Vec<Value>),
    Break,
    Error(Error),
}

impl ops::Try for Result {
    type Output = Value;
    type Residual = Result<Infallible>;

    fn from_output(value: Value) -> Self {
        Result::Value(value)
    }

    fn branch(self) -> ops::ControlFlow<Self::Residual, Self::Output> {
        match self {
            Result::Value(value) => ops::ControlFlow::Continue(value),
            Result::Return(value) => ops::ControlFlow::Break(Result::Return(value)),
            Result::Break => ops::ControlFlow::Break(Result::Break),
            Result::Error(error) => ops::ControlFlow::Break(Result::Error(error)),
        }
    }
}

impl ops::FromResidual for Result {
    fn from_residual(residual: Result<Infallible>) -> Result {
        match residual {
            Result::Value(_) => panic!(),
            Result::Return(value) => Result::Return(value),
            Result::Break => Result::Break,
            Result::Error(error) => Result::Error(error),
        }
    }
}

impl Into<std::result::Result<Value, Error>> for Result {
    fn into(self) -> std::result::Result<Value, Error> {
        match self {
            Result::Value(value) => Ok(value),
            Result::Return(_) => Err(Error::UncaughtReturn),
            Result::Break => Err(Error::UncaughtBreak),
            Result::Error(error) => Err(error),
        }
    }
}

impl Eval for Root {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        for stmt in self.block() {
            stmt.eval(ctx)?;
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for Stmt {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        match self {
            Self::Decl(decl) => decl.eval(ctx),
            Self::Assign(assign) => assign.eval(ctx),
            Self::Func(func) => func.eval(ctx),
            Self::Expr(expr) => expr.eval(ctx),
            Self::Break(r#break) => r#break.eval(ctx),
            Self::Return(r#return) => r#return.eval(ctx),
            Self::Do(r#do) => r#do.eval(ctx),
            Self::While(r#while) => r#while.eval(ctx),
            Self::Repeat(repeat) => repeat.eval(ctx),
            Self::If(r#if) => r#if.eval(ctx),
            Self::ForNum(for_num) => for_num.eval(ctx),
            Self::ForGen(for_gen) => for_gen.eval(ctx),
        }
    }
}

impl Eval for Decl {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for Assign {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for Func {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for Expr {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for Break {
    fn eval(&self, _ctx: &mut Ctx) -> Result {
        Result::Break
    }
}

impl Eval for Return {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        let mut values = Vec::new();
        for expr in self.exprs().unwrap() {
            values.push(expr.eval(ctx)?);
        }

        Result::Return(values)
    }
}

impl Eval for Do {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        for stmt in self.stmts() {
            stmt.eval(ctx)?;
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for While {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        while self.cond().unwrap().eval(ctx)?.op_eq(Value::from_bool(true)) {
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for Repeat {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        loop {
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }

            if self.cond().unwrap().eval(ctx)?.op_eq(Value::from_bool(false)) {
                break;
            }
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for If {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for ForNum {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}

impl Eval for ForGen {
    fn eval(&self, ctx: &mut Ctx) -> Result {
        todo!()
    }
}
