#[derive(Clone, Debug)]
pub enum Stmt {
    Empty,
    Expr(Expr),
    Block(Vec<Stmt>),
    VarDecl {
        name: String,
        init: Option<Expr>,
    },
    LetDecl {
        name: String,
        init: Option<Expr>,
    },
    ConstDecl {
        name: String,
        init: Expr,
    },
    If {
        test: Expr,
        consequent: Box<Stmt>,
        alternate: Option<Box<Stmt>>,
    },
    While {
        test: Expr,
        body: Box<Stmt>,
    },
    For {
        init: Option<Box<Stmt>>,
        test: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },
    ForIn {
        left: String,
        right: Expr,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Throw(Expr),
    Try {
        body: Box<Stmt>,
        catch: Option<(String, Box<Stmt>)>,
        finally: Option<Box<Stmt>>,
    },
    FunctionDecl {
        name: String,
        params: Vec<String>,
        body: Box<Stmt>,
    },
    BlockStmt(Vec<Stmt>),
}

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Lit),
    Identifier(String),
    This,
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expr>,
        prefix: bool,
    },
    Assignment {
        op: AssignOp,
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    New {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Member {
        object: Box<Expr>,
        property: Box<Expr>,
        computed: bool,
    },
    Conditional {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
    },
    FunctionExpr {
        name: Option<String>,
        params: Vec<String>,
        body: Box<Stmt>,
    },
}

#[derive(Clone, Debug)]
pub enum Lit {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    Undefined,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    StrictEq,
    Ne,
    StrictNe,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    UShr,
    In,
    Instanceof,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
    TypeOf,
    Void,
    Delete,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PowAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    ShlAssign,
    ShrAssign,
    UShrAssign,
}
