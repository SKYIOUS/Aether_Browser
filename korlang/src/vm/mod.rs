use std::sync::{Arc, Mutex};
use std::collections::HashMap;
#[derive(Debug, Clone)]
pub enum OpCode { Push(Value), Load(String), Store(String), CreateElement(String), SetProperty(String), AddChild }
#[derive(Debug, Clone)]
pub enum Value { String(String), Number(f64), Bool(bool), Object(Arc<Mutex<KorObject>>), None }
#[derive(Debug)]
pub struct KorObject { pub tag: String, pub properties: HashMap<String, Value>, pub children: Vec<Value> }
pub struct VirtualMachine { pub stack: Vec<Value>, pub heap: HashMap<String, Value> }
impl VirtualMachine {
    pub fn new() -> Self { Self { stack: Vec::new(), heap: HashMap::new() } }
    pub fn execute(&mut self, _bc: Vec<OpCode>) {}
    pub fn update_state(&mut self, name: &str, value: Value) { self.heap.insert(name.to_string(), value); }
}
