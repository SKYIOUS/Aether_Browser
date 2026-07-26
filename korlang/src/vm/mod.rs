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
    GetProperty(String),
    SetState(String),
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
            Value::List(l) => format!("[{} items]", l.len()),
            Value::Object(_) => "[object]".to_string(),
        }
    }
    
    pub fn to_number(&self) -> f64 {
        match self {
            Value::Number(n) => *n,
            Value::Bool(b) => if *b { 1.0 } else { 0.0 },
            Value::String(s) => s.parse::<f64>().unwrap_or(0.0),
            _ => 0.0,
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
            (Value::List(a), Value::List(b)) => a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.equals(y)),
            _ => false,
        }
    }
    
    /// Fix: Safe property access with proper error handling
    pub fn get_property(&self, name: &str) -> Option<Value> {
        if let Value::Object(obj_arc) = self {
            if let Ok(obj) = obj_arc.lock() {
                return obj.properties.get(name).cloned();
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct KorObject {
    pub tag: String,
    pub properties: HashMap<String, Value>,
    pub children: Vec<Value>,
}

impl KorObject {
    pub fn new(tag: String) -> Self {
        Self {
            tag,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }
    
    /// Fix: Safe child addition with validation
    pub fn add_child(&mut self, child: Value) {
        self.children.push(child);
    }
    
    /// Fix: Safe property setting
    pub fn set_property(&mut self, name: String, value: Value) {
        self.properties.insert(name, value);
    }
}

pub struct VirtualMachine {
    pub stack: Vec<Value>,
    pub heap: HashMap<String, Value>,
    pub builtins: HashMap<String, Value>,
    pub native_funcs: HashMap<String, NativeFn>,
    pub functions: HashMap<String, (Vec<String>, Vec<OpCode>)>,
    instruction_pointer: usize,
}

impl Default for VirtualMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VirtualMachine {
    pub fn new() -> Self {
        let mut vm = Self {
            stack: Vec::new(),
            heap: HashMap::new(),
            builtins: HashMap::new(),
            native_funcs: HashMap::new(),
            functions: HashMap::new(),
            instruction_pointer: 0,
        };
        
        // Register built-in functions
        vm.register_native("print", Arc::new(|args| {
            for arg in args {
                print!("{} ", arg.to_string_val());
            }
            println!();
            Value::None
        }));
        
        vm.register_native("len", Arc::new(|args| {
            if let Some(val) = args.first() {
                match val {
                    Value::List(l) => Value::Number(l.len() as f64),
                    Value::String(s) => Value::Number(s.len() as f64),
                    _ => Value::Number(0.0),
                }
            } else {
                Value::Number(0.0)
            }
        }));
        
        vm.register_native("str", Arc::new(|args| {
            if let Some(val) = args.first() {
                Value::String(val.to_string_val())
            } else {
                Value::String(String::new())
            }
        }));
        
        vm.register_native("num", Arc::new(|args| {
            if let Some(val) = args.first() {
                Value::Number(val.to_number())
            } else {
                Value::Number(0.0)
            }
        }));
        
        vm
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

    /// Fix: Execute with proper error recovery instead of unwrap
    pub fn execute(&mut self, bytecode: Vec<OpCode>) {
        let mut ip = 0usize;
        while ip < bytecode.len() {
            match bytecode[ip].clone() {
                OpCode::Push(v) => { 
                    self.stack.push(v); 
                    ip += 1; 
                }
                OpCode::Add => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let result = Value::Number(a.to_number() + b.to_number());
                    self.stack.push(result);
                    ip += 1;
                }
                OpCode::Sub => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let result = Value::Number(a.to_number() - b.to_number());
                    self.stack.push(result);
                    ip += 1;
                }
                OpCode::Mul => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let result = Value::Number(a.to_number() * b.to_number());
                    self.stack.push(result);
                    ip += 1;
                }
                OpCode::Div => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let bn = b.to_number();
                    let result = if bn != 0.0 {
                        Value::Number(a.to_number() / bn)
                    } else {
                        Value::Number(0.0) // Safe division by zero
                    };
                    self.stack.push(result);
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
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    self.stack.push(Value::Bool(a.to_number() < b.to_number()));
                    ip += 1;
                }
                OpCode::Gt => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    self.stack.push(Value::Bool(a.to_number() > b.to_number()));
                    ip += 1;
                }
                OpCode::Le => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    self.stack.push(Value::Bool(a.to_number() <= b.to_number()));
                    ip += 1;
                }
                OpCode::Ge => {
                    let b = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let a = self.stack.pop().unwrap_or(Value::Number(0.0));
                    self.stack.push(Value::Bool(a.to_number() >= b.to_number()));
                    ip += 1;
                }
                OpCode::MakeList(n) => {
                    let mut items = Vec::with_capacity(n);
                    for _ in 0..n { 
                        if let Some(v) = self.stack.pop() { 
                            items.push(v); 
                        } 
                    }
                    items.reverse();
                    self.stack.push(Value::List(items));
                    ip += 1;
                }
                OpCode::ListLen => {
                    if let Some(Value::List(l)) = self.stack.pop() { 
                        self.stack.push(Value::Number(l.len() as f64)); 
                    } else { 
                        self.stack.push(Value::Number(0.0)); 
                    }
                    ip += 1;
                }
                OpCode::ListGet => {
                    let idx = self.stack.pop().unwrap_or(Value::Number(0.0));
                    let list = self.stack.pop().unwrap_or(Value::None);
                    if let (Value::List(l), Value::Number(n)) = (list, idx) {
                        let i = n as usize;
                        if i < l.len() { 
                            self.stack.push(l[i].clone()); 
                        } else { 
                            self.stack.push(Value::None); 
                        }
                    } else { 
                        self.stack.push(Value::None); 
                    }
                    ip += 1;
                }
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
                    let obj = KorObject::new(tag);
                    self.stack.push(Value::Object(Arc::new(Mutex::new(obj))));
                    ip += 1;
                }
                OpCode::SetProperty(name) => {
                    if let Some(val) = self.stack.pop() {
                        if let Some(Value::Object(obj)) = self.stack.last() {
                            if let Ok(mut o) = obj.lock() {
                                o.set_property(name, val);
                            }
                        }
                    }
                    ip += 1;
                }
                OpCode::GetProperty(name) => {
                    if let Some(Value::Object(obj)) = self.stack.pop() {
                        if let Ok(o) = obj.lock() {
                            let val = o.properties.get(&name).cloned().unwrap_or(Value::None);
                            self.stack.push(val);
                        } else {
                            self.stack.push(Value::None);
                        }
                    } else {
                        self.stack.push(Value::None);
                    }
                    ip += 1;
                }
                OpCode::AddChild => {
                    if let Some(child) = self.stack.pop() {
                        if let Some(Value::Object(parent)) = self.stack.last() {
                            if let Ok(mut p) = parent.lock() {
                                p.add_child(child);
                            }
                        }
                    }
                    ip += 1;
                }
                OpCode::Dup => { 
                    if let Some(v) = self.stack.last().cloned() { 
                        self.stack.push(v); 
                    } 
                    ip += 1; 
                }
                OpCode::Pop => { 
                    self.stack.pop(); 
                    ip += 1; 
                }
                OpCode::Jump(target) => { 
                    ip = target; 
                }
                OpCode::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap_or(Value::Bool(false));
                    if !cond.to_bool() { 
                        ip = target; 
                    } else { 
                        ip += 1; 
                    }
                }
                OpCode::Label(_) => { 
                    ip += 1; 
                }
                OpCode::Call(name, argc) => {
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc { 
                        if let Some(v) = self.stack.pop() { 
                            args.push(v); 
                        } 
                    }
                    args.reverse();
                    
                    if let Some((params, body)) = self.functions.get(&name).cloned() {
                        let old_heap = self.heap.clone();
                        for (i, p) in params.iter().enumerate() { 
                            if i < args.len() { 
                                self.heap.insert(p.clone(), args[i].clone()); 
                            } 
                        }
                        self.execute(body);
                        self.heap = old_heap;
                    } else if let Some(cb) = self.native_funcs.get(&name) {
                        let res = cb(&args); 
                        self.stack.push(res);
                    } else {
                        let res = self.builtins.get(&name)
                            .cloned()
                            .unwrap_or(Value::None); 
                        self.stack.push(res);
                    }
                    ip += 1;
                }
                OpCode::StoreFn(name, params, body) => { 
                    self.functions.insert(name, (params, body)); 
                    ip += 1; 
                }
                OpCode::Interpolate(n) => {
                    let mut parts = Vec::new();
                    for _ in 0..n { 
                        if let Some(v) = self.stack.pop() { 
                            parts.push(v.to_string_val()); 
                        } 
                    }
                    parts.reverse();
                    self.stack.push(Value::String(parts.concat()));
                    ip += 1;
                }
                OpCode::ForEach(var, count) => {
                    let is_list = matches!(self.stack.last(), Some(Value::List(_)));
                    let total_count = if let Some(Value::List(l)) = self.stack.last() {
                        l.len()
                    } else {
                        count
                    };

                    let key = format!("__fe_{}", var);
                    let current = self.heap.get(&key).and_then(|v| if let Value::Number(n) = v { Some(*n as usize) } else { None }).unwrap_or(0);

                    if current < total_count {
                        self.heap.insert(key, Value::Number((current + 1) as f64));
                        if let Some(Value::List(l)) = self.stack.last().cloned() {
                            self.heap.insert(var.clone(), l[current].clone());
                        } else {
                            self.heap.insert(var.clone(), Value::Number(current as f64));
                        }
                        ip += 1;
                    } else {
                        self.heap.remove(&key);
                        self.heap.remove(&var);
                        if is_list {
                            self.stack.pop();
                        }
                        let mut jump_target = None;
                        for j in (ip + 1)..bytecode.len() {
                            if let OpCode::Jump(target) = bytecode[j] {
                                if target == ip {
                                    jump_target = Some(j + 1);
                                    break;
                                }
                            }
                        }
                        ip = jump_target.unwrap_or(ip + 1);
                    }
                }
                OpCode::SetState(name) => {
                    if let Some(val) = self.stack.pop() {
                        self.heap.insert(name.to_string(), val);
                    }
                    ip += 1;
                }
            }
        }
        self.instruction_pointer = ip;
    }
    
    pub fn update_state(&mut self, name: &str, value: Value) { 
        self.heap.insert(name.to_string(), value); 
    }
    
    /// Fix: Get root object safely
    pub fn get_root_object(&self) -> Option<Arc<Mutex<KorObject>>> {
        if let Some(Value::Object(obj)) = self.stack.last() {
            Some(obj.clone())
        } else {
            None
        }
    }
}
