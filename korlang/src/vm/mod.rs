use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum OpCode {
    Push(Value),
    Load(String),
    Store(String),
    CreateElement(String),
    SetProperty(String),
    AddChild,
}

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    Object(Arc<Mutex<KorObject>>),
    None,
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
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            heap: HashMap::new(),
        }
    }

    pub fn execute(&mut self, bytecode: Vec<OpCode>) {
        for op in bytecode {
            match op {
                OpCode::Push(v) => self.stack.push(v),
                OpCode::Load(name) => {
                    let v = self.heap.get(&name).cloned().unwrap_or(Value::None);
                    self.stack.push(v);
                }
                OpCode::Store(name) => {
                    if let Some(v) = self.stack.pop() {
                        self.heap.insert(name, v);
                    }
                }
                OpCode::CreateElement(tag) => {
                    let obj = KorObject {
                        tag,
                        properties: HashMap::new(),
                        children: Vec::new(),
                    };
                    self.stack.push(Value::Object(Arc::new(Mutex::new(obj))));
                }
                OpCode::SetProperty(name) => {
                    let val = self.stack.pop().unwrap_or(Value::None);
                    if let Some(Value::Object(obj)) = self.stack.last() {
                        obj.lock().unwrap().properties.insert(name, val);
                    }
                }
                OpCode::AddChild => {
                    let child = self.stack.pop().unwrap_or(Value::None);
                    if let Some(Value::Object(parent)) = self.stack.last() {
                        parent.lock().unwrap().children.push(child);
                    }
                }
            }
        }
    }

    pub fn update_state(&mut self, name: &str, value: Value) {
        self.heap.insert(name.to_string(), value);
    }
}
