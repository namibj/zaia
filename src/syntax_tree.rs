#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    SimpleExpr(SimpleExpr),
    Do(Do),
    While(While),
    Repeat(Repeat),
    If(If),
    ForNumeric(ForNumeric),
    ForGeneric(ForGeneric),
    Return(Return),
    Declare(Declare),
    Assign(Assign),
    Break,
}

#[derive(Debug, PartialEq)]
pub struct Do {
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Expr,
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct Repeat {
    pub block: Vec<Stmt>,
    pub condition: Expr,
}

#[derive(Debug, PartialEq)]
pub struct If {
    pub condition: Expr,
    pub block: Vec<Stmt>,
    pub or: Option<Box<IfChain>>,
}

#[derive(Debug, PartialEq)]
pub enum IfChain {
    ElseIf(If),
    Else(Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub struct ForNumeric {
    pub variable: Ident,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct ForGeneric {
    pub targets: Vec<Ident>,
    pub yielders: Vec<Expr>,
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub values: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Label {
    pub ident: Ident,
}

#[derive(Debug, PartialEq)]
pub enum SimpleExpr {
    Ident(Ident),
    Property(Box<Self>, Expr),
    Method(Box<Self>, Ident),
    FunctionCall(FunctionCall),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Ident(Ident),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Function(Function),
    Literal(Literal),
    FunctionCall(Box<FunctionCall>),
    Table(Table),
    VarArg,
}

impl From<SimpleExpr> for Expr {
    fn from(simple_expr: SimpleExpr) -> Self {
        match simple_expr {
            SimpleExpr::Ident(ident) => Expr::Ident(ident),
            SimpleExpr::Property(simple_expr, index) => Expr::Binary(Box::new(BinaryExpr {
                lhs: Expr::from(*simple_expr),
                op: BinaryOp::Property,
                rhs: index,
            })),
            SimpleExpr::Method(simple_expr, method) => Expr::Binary(Box::new(BinaryExpr {
                lhs: Expr::from(*simple_expr),
                op: BinaryOp::Method,
                rhs: Expr::Ident(method),
            })),
            SimpleExpr::FunctionCall(function_call) => Expr::FunctionCall(Box::new(function_call)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub target: Vec<SimpleExpr>,
    pub value: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Declare {
    pub declarations: Vec<Declaration>,
    pub assign: Option<Assign>,
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: Ident,
    pub is_const: bool,
    pub has_finalizer: bool,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub elements: Vec<TableElement>,
}

#[derive(Debug, PartialEq)]
pub struct TableElement {
    pub key: Option<Expr>,
    pub value: Expr,
}

#[derive(Debug, PartialEq)]
pub struct FunctionCall {
    pub func: Expr,
    pub args: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub args: Vec<Ident>,
    pub vararg: bool,
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOp {
    Not,
    BitNot,
    Pos,
    Neg,
    Len,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOp {
    Or,
    And,
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Exp,
    Mod,
    BitAnd,
    BitOr,
    LeftShift,
    RightShift,
    Equals,
    Xor,
    NotEquals,
    LesserEquals,
    GreaterEquals,
    Lesser,
    Greater,
    Property,
    Method,
    Concat,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Num(NumLiteral),
    String(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum NumLiteral {
    Int(i32),
    Float(f32),
}
