#[derive(Clone, Debug)]
pub enum Instr {
    // Constants
    Const(u32),
    ConstString(u32),
    ConstBool(u32),
    Undefined,
    Null,
    Pop,

    // Variables
    GetLocal(u32),
    SetLocal(u32),
    GetGlobal(u32),
    SetGlobal(u32),

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,

    // Comparisons
    Eq,
    StrictEq,
    Ne,
    StrictNe,
    Lt,
    Gt,
    LtEq,
    GtEq,

    // Logical
    Not,
    And(usize),
    Or(usize),

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    UShr,
    BitNot,

    // Control flow
    Jmp(usize),
    JmpIf(usize),
    JmpIfNot(usize),

    // Functions
    MakeClosure(u32),
    Call(u32),
    Return,

    // Exceptions
    Try(usize),
    Catch,
    EndTry,
    Throw,

    // Special
    TypeOf,
    Void,
    New(u32),
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub instrs: Vec<Instr>,
    pub constants: Vec<f64>,
    pub strings: Vec<String>,
    pub locals: Vec<String>,
    pub is_closure: bool,
}
