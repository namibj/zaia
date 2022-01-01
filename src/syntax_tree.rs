pub struct SyntaxTree {
    pub block: Vec<Stmt>,
}

pub enum Stmt {
    Expr(Expr),
    Assign(Assign),
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
    pub step: Expr,
}

pub struct ForGeneric {
    pub assign: Assign,
    pub block: Vec<Stmt>,
}

pub struct Return {
    pub value: Vec<Expr>,
}

pub struct Label {
    pub ident: Ident,
}

pub struct Assign {
    pub is_local: bool,
    pub is_const: bool,
    pub target: Vec<Expr>,
    pub value: Expr,
}

pub enum Expr {
    Variable(Ident),
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Function(Function),
    Literal(Literal),
    FunctionCall(Box<FunctionCall>),
    Table(Table),
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
}

pub struct Ident {
    pub name: String,
}

pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Box<Expr>,
}

pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
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
    Rem,
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
}

pub enum Literal {
    Nil,
    Boolean(bool),
    Num(NumLiteral),
    String(String),
}

pub enum NumLiteral {
    Integer(i64),
    Float(f64),
}
