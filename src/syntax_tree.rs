#[derive(Debug, PartialEq)]
pub struct SyntaxTree {
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Label(Label),
    Do(Do),
    While(While),
    Repeat(Repeat),
    If(If),
    ForNumeric(ForNumeric),
    ForGeneric(ForGeneric),
    Return(Return),
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
    pub yielder: Expr,
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
pub enum Expr {
    Variable(Ident),
    Unary(Box<UnaryExpr>),
    Binary(Box<BinaryExpr>),
    Function(Function),
    Literal(Literal),
    FunctionCall(Box<FunctionCall>),
    Table(Table),
    Assign(Box<Assign>),
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub is_local: bool,
    pub is_const: bool,
    pub target: Vec<Expr>,
    pub value: Vec<Expr>,
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
    pub block: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
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
    Index,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Nil,
    Boolean(bool),
    Num(NumLiteral),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum NumLiteral {
    Int(i64),
    Float(f64),
}
