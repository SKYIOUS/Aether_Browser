pub mod ast;
pub mod bytecode;
pub mod compile;
pub mod heap;
pub mod lexer;
pub mod parser;
pub mod value;
pub mod vm;

#[cfg(test)]
mod eval_tests;

use std::sync::atomic::{AtomicU64, Ordering};

use crate::heap::Heap;
use crate::value::Value;
use crate::vm::{Ctrl, Vm};

pub struct Realm {
    pub vm: Vm,
}

impl Realm {
    pub fn new() -> Self {
        Self {
            vm: Vm::new(Heap::new()),
        }
    }

    pub fn eval(&mut self, source: &str) -> Result<Value, EvalError> {
        let tokens = lexer::tokenize(source).map_err(|e| EvalError::Syntax(e.to_string()))?;
        let ast = parser::parse(&tokens).map_err(|e| EvalError::Syntax(e.to_string()))?;
        let fun = compile::compile(&ast).map_err(|e| EvalError::Syntax(e.to_string()))?;
        self.vm.exec(fun).map_err(|ctrl| match ctrl {
            Ctrl::Throw(v) => EvalError::Exception(v),
            other => EvalError::Incomplete(other),
        })
    }
}

impl Default for Realm {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum EvalError {
    Syntax(String),
    Exception(Value),
    Incomplete(Ctrl),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::Syntax(msg) => write!(f, "SyntaxError: {msg}"),
            EvalError::Exception(v) => write!(f, "Uncaught {v}"),
            EvalError::Incomplete(c) => write!(f, "Incomplete: {c:?}"),
        }
    }
}

impl std::error::Error for EvalError {}

pub static REALM_ID: AtomicU64 = AtomicU64::new(1);

pub fn next_realm_id() -> u64 {
    REALM_ID.fetch_add(1, Ordering::Relaxed)
}
