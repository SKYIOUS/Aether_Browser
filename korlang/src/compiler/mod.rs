use crate::vm::OpCode;
pub mod lexer; pub mod parser; pub mod rust_gen;
pub fn compile(_s: &str) -> Vec<OpCode> { vec![] }
