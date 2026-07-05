//! Stack-based VM that executes Korlang bytecode.
//! Operates on a stack of [`Value`] with a persistent [`heap`](Self::heap)
//! for variables and loop state. Elements are created as [`KorObject`] nodes
//! and assembled via `AddChild`, producing a UI component tree.

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// ponytail: simple native fn pointer; replace with proper FFI when more than ~5 callbacks
pub type NativeFn = Arc<dyn Fn(&[Value]) -> Value + Send + Sync>;

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(Value),
    Load(String),
    Store(String),
    CreateElement(String),
    SetProperty(String),
    AddChild,
    Jump(usize),
    JumpIfFalse(usize),
    Label(String),
    Call(String, usize),
    Interpolate(usize),
    ForEach(String, usize),
    Dup,
    Pop,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Object(Arc<Mutex<KorObject>>),
    None,
}

impl Value {
    pub fn to_string_val(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::None => "none".to_string(),
            Value::Object(_) => "[object]".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct KorObject {
    pub tag: String,
    pub properties: HashMap<String, Value>,
    pub children: Vec<Value>,
}

pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub heap: HashMap<String, Value>,
    pub builtins: HashMap<String, Value>,
    pub native_funcs: HashMap<String, NativeFn>,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
            builtins: HashMap::new(),
            native_funcs: HashMap::new(),
            instruction_pointer: 0,
        }
    }

    pub fn set_builtin(&mut self, name: &str, value: Value) {
        self.builtins.insert(name.to_string(), value);
    }

    pub fn get_builtin(&self, name: &str) -> Option<&Value> {
        self.builtins.get(name)
    }

    pub fn register_native(&mut self, name: &str, f: NativeFn) {
        self.native_funcs.insert(name.to_string(), f);
    }

    pub fn execute(&mut self, bytecode: Vec<OpCode>) {
        let mut ip = 0usize;
        while ip < bytecode.len() {
            match bytecode[ip].clone() {
                OpCode::Push(v) => { self.stack.push(v); ip += 1; }
                OpCode::Load(name) => {
                    let v = self.heap.get(&name)
                        .or_else(|| self.builtins.get(&name))
                        .cloned()
                        .unwrap_or(Value::None);
                    self.stack.push(v);
                    ip += 1;
                }
                OpCode::Store(name) => {
                    if let Some(v) = self.stack.pop() {
                        self.heap.insert(name, v);
                    }
                    ip += 1;
                }
                OpCode::CreateElement(tag) => {
                    let obj = KorObject {
                        tag,
                        properties: HashMap::new(),
                        children: Vec::new(),
                    };
                    self.stack.push(Value::Object(Arc::new(Mutex::new(obj))));
                    ip += 1;
                }
                OpCode::SetProperty(name) => {
                    if self.stack.len() < 2 {
                        eprintln!("vm: stack underflow in SetProperty (need 2, have {})", self.stack.len());
                        ip += 1; continue;
                    }
                    let val = self.stack.pop().unwrap();
                    if let Some(Value::Object(obj)) = self.stack.last() {
                        obj.lock().unwrap_or_else(|e| e.into_inner()).properties.insert(name, val);
                    }
                    ip += 1;
                }
                OpCode::AddChild => {
                    if self.stack.len() < 2 {
                        eprintln!("vm: stack underflow in AddChild (need 2, have {})", self.stack.len());
                        ip += 1; continue;
                    }
                    let child = self.stack.pop().unwrap();
                    if let Some(Value::Object(parent)) = self.stack.last() {
                        parent.lock().unwrap_or_else(|e| e.into_inner()).children.push(child);
                    }
                    ip += 1;
                }
                OpCode::Dup => {
                    if self.stack.is_empty() {
                        eprintln!("vm: stack underflow in Dup");
                        ip += 1; continue;
                    }
                    let v = self.stack.last().unwrap().clone();
                    self.stack.push(v);
                    ip += 1;
                }
                OpCode::Pop => {
                    if self.stack.is_empty() {
                        eprintln!("vm: stack underflow in Pop");
                        ip += 1; continue;
                    }
                    self.stack.pop();
                    ip += 1;
                }
                OpCode::Jump(target) => {
                    if target >= bytecode.len() {
                        eprintln!("vm: Jump target {} out of bounds (len {})", target, bytecode.len());
                        ip += 1; continue;
                    }
                    ip = target;
                }
                OpCode::JumpIfFalse(target) => {
                    if target >= bytecode.len() {
                        eprintln!("vm: JumpIfFalse target {} out of bounds (len {})", target, bytecode.len());
                        ip += 1; continue;
                    }
                    let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                    let is_false = match cond {
                        Value::Bool(b) => !b,
                        Value::Number(n) => n == 0.0,
                        Value::String(s) => s.is_empty(),
                        Value::None => true,
                        Value::Object(_) => false,
                    };
                    if is_false { ip = target; } else { ip += 1; }
                }
                OpCode::Label(_) => { ip += 1; }
                OpCode::Call(name, argc) => {
                    // ponytail: pop args (top = last arg), reverse for correct order
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        if let Some(v) = self.stack.pop() { args.push(v); } else { break; }
                    }
                    args.reverse();
                    for (i, v) in args.iter().enumerate() {
                        self.heap.insert(format!("__arg_{}", i), v.clone());
                    }
                    let result = if let Some(cb) = self.native_funcs.get(&name) {
                        cb(&args)
                    } else {
                        self.builtins.get(&name).cloned().unwrap_or(Value::None)
                    };
                    self.stack.push(result);
                    ip += 1;
                }
                OpCode::Interpolate(n) => {
                    let mut parts: Vec<String> = Vec::new();
                    for _ in 0..n {
                        if let Some(v) = self.stack.pop() {
                            parts.push(v.to_string_val());
                        }
                    }
                    parts.reverse();
                    let result = parts.concat();
                    self.stack.push(Value::String(result));
                    ip += 1;
                }
                OpCode::ForEach(var, count) => {
                    let key = format!("__fe_{}", var);
                    let end_key = format!("__fe_end_{}", var);
                    let current = self.heap.get(&key).and_then(|v| {
                        if let Value::Number(n) = v { Some(*n as usize) } else { None }
                    }).unwrap_or(0);

                    if current < count {
                        // ponytail: scan once to cache the end-jump offset
                        if current == 0 {
                            let mut scan = ip + 1;
                            while scan < bytecode.len() {
                                if let OpCode::Jump(target) = &bytecode[scan] {
                                    if *target == ip {
                                        self.heap.insert(end_key, Value::Number((scan + 1) as f64));
                                        break;
                                    }
                                }
                                scan += 1;
                            }
                        }
                        self.heap.insert(key, Value::Number((current + 1) as f64));
                        self.heap.insert(var.clone(), Value::Number(current as f64));
                        ip += 1;
                    } else {
                        self.heap.remove(&key);
                        ip = self.heap.remove(&end_key).and_then(|v| {
                            if let Value::Number(n) = v { Some(n as usize) } else { None }
                        }).unwrap_or(ip + 1);
                    }
                }
            }
        }
        self.instruction_pointer = ip;
    }

    pub fn update_state(&mut self, name: &str, value: Value) {
        self.heap.insert(name.to_string(), value);
    }
}
