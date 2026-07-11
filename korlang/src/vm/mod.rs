//! Stack-based VM that executes Korlang bytecode.
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

pub type NativeFn = Arc<dyn Fn(&[Value]) -> Value + Send + Sync>;

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(Value),
    Load(String),
    Store(String),
    CreateElement(String),
    SetProperty(String),
    AddChild,
    Add, Sub, Mul, Div,
    And, Or, Not,
    Eq, Neq, Lt, Gt, Le, Ge,
    MakeList(usize), ListLen, ListGet,
    Jump(usize),
    JumpIfFalse(usize),
    Label(String),
    Call(String, usize),
    StoreFn(String, Vec<String>, Vec<OpCode>),
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
    List(Vec<Value>),
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
            Value::List(_) => "[list]".to_string(),
            Value::Object(_) => "[object]".to_string(),
        }
    }
    pub fn to_bool(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::None => false,
            Value::List(l) => !l.is_empty(),
            Value::Object(_) => true,
        }
    }
    pub fn equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::None, Value::None) => true,
            _ => false,
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
    pub functions: HashMap<String, (Vec<String>, Vec<OpCode>)>,
    instruction_pointer: usize,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
            builtins: HashMap::new(),
            native_funcs: HashMap::new(),
            functions: HashMap::new(),
            instruction_pointer: 0,
        }
    }
    pub fn set_builtin(&mut self, name: &str, value: Value) { self.builtins.insert(name.to_string(), value); }
    pub fn get_builtin(&self, name: &str) -> Option<&Value> { self.builtins.get(name) }
    pub fn register_native(&mut self, name: &str, f: NativeFn) { self.native_funcs.insert(name.to_string(), f); }

    pub fn execute(&mut self, bytecode: Vec<OpCode>) {
        let mut ip = 0usize;
        while ip < bytecode.len() {
            match bytecode[ip].clone() {
                OpCode::Push(v) => { self.stack.push(v); ip += 1; }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Number(na + nb)); }
                    else { self.stack.push(Value::None); }
                    ip += 1;
                }
                OpCode::Sub => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Number(na - nb)); }
                    else { self.stack.push(Value::None); }
                    ip += 1;
                }
                OpCode::Mul => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Number(na * nb)); }
                    else { self.stack.push(Value::None); }
                    ip += 1;
                }
                OpCode::Div => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Number(if nb != 0.0 { na / nb } else { 0.0 })); }
                    else { self.stack.push(Value::None); }
                    ip += 1;
                }
                OpCode::And => {
                    let b = self.stack.pop().unwrap_or(Value::Bool(false));
                    let a = self.stack.pop().unwrap_or(Value::Bool(false));
                    self.stack.push(Value::Bool(a.to_bool() && b.to_bool()));
                    ip += 1;
                }
                OpCode::Or => {
                    let b = self.stack.pop().unwrap_or(Value::Bool(false));
                    let a = self.stack.pop().unwrap_or(Value::Bool(false));
                    self.stack.push(Value::Bool(a.to_bool() || b.to_bool()));
                    ip += 1;
                }
                OpCode::Not => {
                    let a = self.stack.pop().unwrap_or(Value::Bool(false));
                    self.stack.push(Value::Bool(!a.to_bool()));
                    ip += 1;
                }
                OpCode::Eq => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    self.stack.push(Value::Bool(a.equals(&b)));
                    ip += 1;
                }
                OpCode::Neq => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    self.stack.push(Value::Bool(!a.equals(&b)));
                    ip += 1;
                }
                OpCode::Lt => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Bool(na < nb)); }
                    else { self.stack.push(Value::Bool(false)); }
                    ip += 1;
                }
                OpCode::Gt => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Bool(na > nb)); }
                    else { self.stack.push(Value::Bool(false)); }
                    ip += 1;
                }
                OpCode::Le => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Bool(na <= nb)); }
                    else { self.stack.push(Value::Bool(false)); }
                    ip += 1;
                }
                OpCode::Ge => {
                    let b = self.stack.pop().unwrap_or(Value::None);
                    let a = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::Number(na), Value::Number(nb)) = (a, b) { self.stack.push(Value::Bool(na >= nb)); }
                    else { self.stack.push(Value::Bool(false)); }
                    ip += 1;
                }
                OpCode::MakeList(n) => {
                    let mut items = Vec::with_capacity(n);
                    for _ in 0..n { if let Some(v) = self.stack.pop() { items.push(v); } }
                    items.reverse();
                    self.stack.push(Value::List(items));
                    ip += 1;
                }
                OpCode::ListLen => {
                    if let Some(Value::List(l)) = self.stack.pop() { self.stack.push(Value::Number(l.len() as f64)); }
                    else { self.stack.push(Value::Number(0.0)); }
                    ip += 1;
                }
                OpCode::ListGet => {
                    let idx = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let list = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::List(l), Value::Number(n)) = (list, idx) {
                        let i = n as usize;
                        if i < l.len() { self.stack.push(l[i].clone()); }
                        else { self.stack.push(Value::None); }
                    } else { self.stack.push(Value::None); }
                    ip += 1;
                }
                OpCode::Load(name) => {
                    let v = self.heap.get(&name).or_else(|| self.builtins.get(&name)).cloned().unwrap_or(Value::None);
                    self.stack.push(v);
                    ip += 1;
                }
                OpCode::Store(name) => { if let Some(v) = self.stack.pop() { self.heap.insert(name, v); } ip += 1; }
                OpCode::CreateElement(tag) => {
                    let obj = KorObject { tag, properties: HashMap::new(), children: Vec::new() };
                    self.stack.push(Value::Object(Arc::new(Mutex::new(obj))));
                    ip += 1;
                }
                OpCode::SetProperty(name) => {
                    let val = if let Some(v) = self.stack.pop() { v } else { ip += 1; continue; };
                    if let Some(Value::Object(obj)) = self.stack.last() { obj.lock().unwrap().properties.insert(name, val); }
                    ip += 1;
                }
                OpCode::AddChild => {
                    let child = if let Some(v) = self.stack.pop() { v } else { ip += 1; continue; };
                    if let Some(Value::Object(parent)) = self.stack.last() { parent.lock().unwrap().children.push(child); }
                    ip += 1;
                }
                OpCode::Dup => { if let Some(v) = self.stack.last().cloned() { self.stack.push(v); } ip += 1; }
                OpCode::Pop => { self.stack.pop(); ip += 1; }
                OpCode::Jump(target) => { ip = target; }
                OpCode::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                    if !cond.to_bool() { ip = target; } else { ip += 1; }
                }
                OpCode::Label(_) => { ip += 1; }
                OpCode::Call(name, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc { if let Some(v) = self.stack.pop() { args.push(v); } }
                    args.reverse();
                    if let Some((params, body)) = self.functions.get(&name).cloned() {
                        let old_heap = self.heap.clone();
                        for (i, p) in params.iter().enumerate() { if i < args.len() { self.heap.insert(p.clone(), args[i].clone()); } }
                        self.execute(body);
                        self.heap = old_heap;
                    } else if let Some(cb) = self.native_funcs.get(&name) {
                        let res = cb(&args); self.stack.push(res);
                    } else {
                        let res = self.builtins.get(&name).cloned().unwrap_or(Value::None); self.stack.push(res);
                    }
                    ip += 1;
                }
                OpCode::StoreFn(name, params, body) => { self.functions.insert(name, (params, body)); ip += 1; }
                OpCode::Interpolate(n) => {
                    let mut parts = Vec::new();
                    for _ in 0..n { if let Some(v) = self.stack.pop() { parts.push(v.to_string_val()); } }
                    parts.reverse();
                    self.stack.push(Value::String(parts.concat()));
                    ip += 1;
                }
                OpCode::ForEach(var, end_ip) => {
                    let key_coll = format!("__fe_coll_{}", var);
                    let key_idx = format!("__fe_idx_{}", var);
                    if !self.heap.contains_key(&key_coll) {
                        if let Some(coll @ Value::List(_)) = self.stack.pop() {
                            self.heap.insert(key_coll.clone(), coll);
                            self.heap.insert(key_idx.clone(), Value::Number(0.0));
                        } else {
                            ip = end_ip;
                            continue;
                        }
                    }
                    let (is_done, val) = {
                        let coll = self.heap.get(&key_coll).unwrap();
                        let idx_val = self.heap.get(&key_idx).unwrap();
                        if let (Value::List(l), Value::Number(n)) = (coll, idx_val) {
                            let idx = *n as usize;
                            if idx < l.len() { (false, Some(l[idx].clone())) } else { (true, None) }
                        } else { (true, None) }
                    };
                    if is_done {
                        self.heap.remove(&key_coll);
                        self.heap.remove(&key_idx);
                        ip = end_ip;
                    } else {
                        if let Some(Value::Number(n)) = self.heap.get(&key_idx).cloned() {
                            self.heap.insert(key_idx, Value::Number(n + 1.0));
                        }
                        if let Some(v) = val { self.heap.insert(var, v); }
                        ip += 1;
                    }
                }
                }
        }
        self.instruction_pointer = ip;
    }
    pub fn update_state(&mut self, name: &str, value: Value) { self.heap.insert(name.to_string(), value); }
}
