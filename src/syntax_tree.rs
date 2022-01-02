pub struct SyntaxTree {
    pub block: Vec<Stmt>,
}

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

pub struct Do {
    pub block: Vec<Stmt>,
}

pub struct While {
    pub condition: Expr,
    pub block: Vec<Stmt>,
}

pub struct Repeat {
    pub block: Vec<Stmt>,
    pub condition: Expr,
}

pub struct If {
    pub condition: Expr,
    pub block: Vec<Stmt>,
    pub or: Option<Box<IfChain>>,
}

pub enum IfChain {
    ElseIf(If),
    Else(Vec<Stmt>),
}

pub struct ForNumeric {
    pub variable: Ident,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
    pub block: Vec<Stmt>,
}

pub struct ForGeneric {
    pub targets: Vec<Ident>,
    pub yielder: Expr,
    pub block: Vec<Stmt>,
}

pub struct Return {
    pub values: Vec<Expr>,
}

pub struct Label {
    pub ident: Ident,
}

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

pub struct Assign {
    pub is_local: bool,
    pub is_const: bool,
    pub target: Vec<Expr>,
    pub value: Vec<Expr>,
}

pub struct Table {
    elements: Vec<TableElement>,
}

pub struct TableElement {
    pub key: Option<Expr>,
    pub value: Expr,
}

pub struct FunctionCall {
    pub func: Expr,
    pub args: Vec<Expr>,
}

pub struct Function {
    pub args: Vec<Ident>,
    pub block: Vec<Stmt>,
}

pub struct Ident {
    pub name: String,
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
}

pub struct BinaryExpr {
    pub op: BinaryOp,
    pub lhs: Expr,
    pub rhs: Expr,
}

pub enum UnaryOp {
    Not,
    BitNot,
    Pos,
    Neg,
    Len,
}

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
    AssocFunction,
    Method,
    Concat,
    Index,
}

pub enum Literal {
    Nil,
    Boolean(bool),
    Num(NumLiteral),
    String(String),
}

pub enum NumLiteral {
    Int(i64),
    Float(f64),
}
