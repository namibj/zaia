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
    fn eval(&self, ctx: &Ctx) -> Result;
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
    fn eval(&self, ctx: &Ctx) -> Result {
        for stmt in self.block() {
            stmt.eval(ctx)?;
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for Stmt {
    fn eval(&self, ctx: &Ctx) -> Result {
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
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for Assign {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for Func {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for Expr {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for Break {
    fn eval(&self, _ctx: &Ctx) -> Result {
        Result::Break
    }
}

impl Eval for Return {
    fn eval(&self, ctx: &Ctx) -> Result {
        let mut values = Vec::new();
        for expr in self.exprs().unwrap() {
            values.push(expr.eval(ctx)?);
        }

        Result::Return(values)
    }
}

impl Eval for Do {
    fn eval(&self, ctx: &Ctx) -> Result {
        let _scope = ctx.scope_push();
        for stmt in self.stmts() {
            stmt.eval(ctx)?;
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for While {
    fn eval(&self, ctx: &Ctx) -> Result {
        while self
            .cond()
            .unwrap()
            .eval(ctx)?
            .op_eq(Value::from_bool(true))
        {
            let _scope = ctx.scope_push();
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for Repeat {
    fn eval(&self, ctx: &Ctx) -> Result {
        loop {
            let _scope = ctx.scope_push();
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }

            if self
                .cond()
                .unwrap()
                .eval(ctx)?
                .op_eq(Value::from_bool(false))
            {
                break;
            }
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for If {
    fn eval(&self, ctx: &Ctx) -> Result {
        if self
            .cond()
            .unwrap()
            .eval(ctx)?
            .op_eq(Value::from_bool(true))
        {
            let _scope = ctx.scope_push();
            for stmt in self.stmts().unwrap() {
                stmt.eval(ctx)?;
            }
        } else if let Some(elif) = self.else_chain() {
            if let Some(el_if) = elif.elseif_block() {
                el_if.eval(ctx)?;
            } else if let Some(el) = elif.else_block() {
                let _scope = ctx.scope_push();
                for stmt in el {
                    stmt.eval(ctx)?;
                }
            }
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for ForNum {
    fn eval(&self, ctx: &Ctx) -> Result {
        let (counter, init) = self.counter().unwrap();
        let init = init.eval(ctx)?;
        let end = self.end().unwrap().eval(ctx)?;
        let step = if let Some(expr) = self.step() {
            expr.eval(ctx)?
        } else {
            Value::from_int(1)
        };

        let _scope = ctx.scope_push();
        let var = ctx.intern_ident(counter);
        ctx.local(var);
        ctx.assign(var, init);

        while !ctx.resolve(var).op_eq(end) {
            let _scope = ctx.scope_push();
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }

            let value = ctx.resolve(var);
            let value = value.op_add(step);
            ctx.assign(var, value);
        }

        Result::Value(Value::from_nil())
    }
}

impl Eval for ForGen {
    fn eval(&self, ctx: &Ctx) -> Result {
        let _scope = ctx.scope_push();

        loop {
            for (target, value) in self.targets().unwrap().zip(self.values().unwrap()) {
                let var = ctx.intern_ident(target);
                let value = value.eval(ctx)?;
                ctx.local(var);
                ctx.assign(var, value);
            }

            let first_target = self.targets().unwrap().next().unwrap();
            let first_var = ctx.intern_ident(first_target);

            if ctx.resolve(first_var).op_eq(Value::from_nil()) {
                break;
            }

            let _scope = ctx.scope_push();
            for stmt in self.block().unwrap() {
                stmt.eval(ctx)?;
            }
        }

        Result::Value(Value::from_nil())
    }
}
