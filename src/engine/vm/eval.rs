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
    Ident,
    Literal,
    BinaryOp,
    PrefixOp,
    FuncExpr,
    Table,
    FuncCall,
    Index,
    PrefixOperator,
    BinaryOperator
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
        match self {
            Self::Ident(ident) => ident.eval(ctx),
            Self::Literal(literal) => literal.eval(ctx),
            Self::Func(func) => func.eval(ctx),
            Self::Table(call) => call.eval(ctx),
            Self::PrefixOp(prefix_op) => prefix_op.eval(ctx),
            Self::BinaryOp(binary_op) => binary_op.eval(ctx),
            Self::FuncCall(call) => call.eval(ctx),
            Self::Index(index) => index.eval(ctx),
            Self::VarArg => panic!("varargs are currently unsupported"),
        }
    }
}

impl Eval for Ident {
    fn eval(&self, ctx: &Ctx) -> Result {
        let key = ctx.intern_ident(self);
        Result::Value(ctx.resolve(key))
    }
}

impl Eval for Literal {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for FuncExpr {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for Table {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for PrefixOp {
    fn eval(&self, ctx: &Ctx) -> Result {
        let rhs = self.rhs().unwrap().eval(ctx)?;

        Result::Value(match self.op().unwrap() {
            PrefixOperator::None => rhs,
            PrefixOperator::Neg => rhs.op_neg(),
            PrefixOperator::Not => rhs.op_not(),
            PrefixOperator::Len => rhs.op_len(),
            PrefixOperator::BitNot => rhs.op_bit_not(),
        })
    }
}

impl Eval for BinaryOp {
    fn eval(&self, ctx: &Ctx) -> Result {
        let lhs = self.lhs().unwrap().eval(ctx)?;
        let rhs = self.rhs().unwrap().eval(ctx)?;

        Result::Value(match self.op().unwrap() {
            BinaryOperator::And => lhs.op_and(rhs),
            BinaryOperator::Or => lhs.op_or(rhs),
            BinaryOperator::Add=> lhs.op_add(rhs),
            BinaryOperator::Sub => lhs.op_sub(rhs),
            BinaryOperator::Mul => lhs.op_mul(rhs),
            BinaryOperator::Div => lhs.op_div(rhs),
            BinaryOperator::IntDiv => lhs.op_int_div(rhs),
            BinaryOperator::Exp => lhs.op_exp(rhs),
            BinaryOperator::Mod => lhs.op_mod(rhs),
            BinaryOperator::BitAnd => lhs.op_bit_and(rhs),
            BinaryOperator::BitOr => lhs.op_bit_or(rhs),
            BinaryOperator::LShift => lhs.op_lshift(rhs),
            BinaryOperator::RShift => lhs.op_rshift(rhs),
            BinaryOperator::Eq => lhs.op_eq(rhs),
            BinaryOperator::BitXor => lhs.op_bit_xor(rhs),
            BinaryOperator::NEq => lhs.op_neq(rhs),
            BinaryOperator::LEq => lhs.op_leq(rhs),
            BinaryOperator::GEq => lhs.op_geq(rhs),
            BinaryOperator::Gt => lhs.op_gt(rhs),
            BinaryOperator::Lt => lhs.op_lt(rhs),
            BinaryOperator::Property => lhs.op_property(rhs),
            BinaryOperator::Method => lhs.op_method(rhs),
            BinaryOperator::Concat => lhs.op_concat(rhs),
        })
    }
}

impl Eval for Index {
    fn eval(&self, ctx: &Ctx) -> Result {
        todo!()
    }
}

impl Eval for FuncCall {
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
            == Value::from_bool(true)
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
                == Value::from_bool(true)
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
            == Value::from_bool(true)
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
        let var = ctx.intern_ident(&counter);
        ctx.local(var);
        ctx.assign(var, init);

        while ctx.resolve(var) != end {
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

        for target in self.targets().unwrap() {
            let var = ctx.intern_ident(&target);
            ctx.local(var);
        }

        loop {
            for (target, value) in self.targets().unwrap().zip(self.values().unwrap()) {
                let var = ctx.intern_ident(&target);
                let value = value.eval(ctx)?;
                ctx.assign(var, value);
            }

            let first_target = self.targets().unwrap().next().unwrap();
            let first_var = ctx.intern_ident(&first_target);

            if ctx.resolve(first_var) == Value::from_nil() {
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
