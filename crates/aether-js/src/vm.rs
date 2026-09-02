use crate::bytecode::{Function, Instr};
use crate::heap::Heap;
use crate::value::Value;

#[derive(Debug)]
pub enum Ctrl {
    Normal,
    Throw(Value),
    Return(Value),
}

impl std::fmt::Display for Ctrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ctrl::Normal => write!(f, "Normal"),
            Ctrl::Throw(v) => write!(f, "Throw({v})"),
            Ctrl::Return(v) => write!(f, "Return({v})"),
        }
    }
}

struct Frame {
    fun: usize,
    locals: Vec<Value>,
    stack: Vec<Value>,
    ip: usize,
}

pub struct Vm {
    pub heap: Heap,
    frames: Vec<Frame>,
    funs: Vec<Function>,
}

impl Vm {
    pub fn new(heap: Heap) -> Self {
        Self {
            heap,
            frames: Vec::new(),
            funs: Vec::new(),
        }
    }

    pub fn exec(&mut self, fun: Function) -> Result<Value, Ctrl> {
        let fun_idx = self.funs.len();
        self.funs.push(fun);
        let f = &self.funs[fun_idx];
        let locals_count = f.locals.len();
        let frame = Frame {
            fun: fun_idx,
            locals: vec![Value::Undefined; locals_count],
            stack: Vec::new(),
            ip: 0,
        };
        self.frames.push(frame);
        self.run_frame()
    }

    fn run_frame(&mut self) -> Result<Value, Ctrl> {
        loop {
            if self.frames.is_empty() {
                return Ok(Value::Undefined);
            }

            let frame_idx = self.frames.len() - 1;
            let fun_idx = self.frames[frame_idx].fun;
            let ip = self.frames[frame_idx].ip;
            let instrs_len = self.funs[fun_idx].instrs.len();

            if ip >= instrs_len {
                let val = self.frames[frame_idx]
                    .stack
                    .last()
                    .cloned()
                    .unwrap_or(Value::Undefined);
                self.frames.pop();
                if self.frames.is_empty() {
                    return Ok(val);
                }
                self.frames.last_mut().unwrap().stack.push(val);
                continue;
            }

            let instr = self.funs[fun_idx].instrs[ip].clone();
            self.frames[frame_idx].ip += 1;

            match instr {
                Instr::Const(slot) => {
                    let val = Value::Number(self.funs[fun_idx].constants[slot as usize]);
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::ConstString(slot) => {
                    let val = Value::String(self.funs[fun_idx].strings[slot as usize].clone());
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::ConstBool(b) => {
                    self.frames[frame_idx].stack.push(Value::Bool(b != 0));
                }
                Instr::Undefined => {
                    self.frames[frame_idx].stack.push(Value::Undefined);
                }
                Instr::Null => {
                    self.frames[frame_idx].stack.push(Value::Null);
                }
                Instr::Pop => {
                    self.frames[frame_idx].stack.pop();
                }
                Instr::GetLocal(slot) => {
                    let val = self.frames[frame_idx].locals[slot as usize].clone();
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::SetLocal(slot) => {
                    let val = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx].locals[slot as usize] = val.clone();
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::GetGlobal(slot) => {
                    let key = self.funs[fun_idx].strings[slot as usize].clone();
                    let val = self
                        .heap
                        .obj_get(self.heap.global(), &key)
                        .cloned()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::SetGlobal(slot) => {
                    let val = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let key = self.funs[fun_idx].strings[slot as usize].clone();
                    self.heap.obj_set(self.heap.global(), key, val.clone());
                    self.frames[frame_idx].stack.push(val);
                }
                Instr::Add => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let result = match (&l, &r) {
                        (Value::String(a), b) => {
                            Value::String(format!("{a}{}", b.to_string_value()))
                        }
                        (a, Value::String(b)) => {
                            Value::String(format!("{}{b}", a.to_string_value()))
                        }
                        _ => Value::Number(l.to_number() + r.to_number()),
                    };
                    self.frames[frame_idx].stack.push(result);
                }
                Instr::Sub => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number(l.to_number() - r.to_number()));
                }
                Instr::Mul => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number(l.to_number() * r.to_number()));
                }
                Instr::Div => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number(l.to_number() / r.to_number()));
                }
                Instr::Mod => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number(l.to_number() % r.to_number()));
                }
                Instr::Neg => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number(-v.to_number()));
                }
                Instr::Not => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(!v.is_truthy()));
                }
                Instr::BitNot => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Number((!(v.to_number() as i32)) as f64));
                }
                Instr::Eq => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() == r.to_number()));
                }
                Instr::StrictEq => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx].stack.push(Value::Bool(l == r));
                }
                Instr::Ne => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() != r.to_number()));
                }
                Instr::StrictNe => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx].stack.push(Value::Bool(l != r));
                }
                Instr::Lt => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() < r.to_number()));
                }
                Instr::Gt => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() > r.to_number()));
                }
                Instr::LtEq => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() <= r.to_number()));
                }
                Instr::GtEq => {
                    let r = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let l = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames[frame_idx]
                        .stack
                        .push(Value::Bool(l.to_number() >= r.to_number()));
                }
                Instr::Jmp(target) => {
                    self.frames[frame_idx].ip = target;
                    continue;
                }
                Instr::JmpIf(target) => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    if v.is_truthy() {
                        self.frames[frame_idx].ip = target;
                        continue;
                    }
                }
                Instr::JmpIfNot(target) => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    if !v.is_truthy() {
                        self.frames[frame_idx].ip = target;
                        continue;
                    }
                }
                Instr::And(alt) => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    if v.is_truthy() {
                        self.frames[frame_idx].stack.push(v);
                    } else {
                        self.frames[frame_idx].ip = alt;
                    }
                }
                Instr::Or(alt) => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    if v.is_truthy() {
                        self.frames[frame_idx].ip = alt;
                    } else {
                        self.frames[frame_idx].stack.push(v);
                    }
                }
                Instr::MakeClosure(slot) => {
                    let name = self.funs[fun_idx].strings[slot as usize].clone();
                    let local_slot =
                        self.funs[fun_idx].locals.len() + self.frames[frame_idx].stack.len();
                    let _ = (name, local_slot);
                    self.frames[frame_idx].stack.push(Value::Undefined);
                }
                Instr::Call(argc) => {
                    let mut args: Vec<Value> = Vec::new();
                    for _ in 0..argc {
                        args.push(
                            self.frames[frame_idx]
                                .stack
                                .pop()
                                .unwrap_or(Value::Undefined),
                        );
                    }
                    args.reverse();
                    let callee = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    match callee {
                        Value::Function(_callee_id) => {
                            self.frames[frame_idx].stack.push(Value::Undefined);
                        }
                        _ => {
                            self.frames[frame_idx].stack.push(Value::Undefined);
                        }
                    }
                }
                Instr::Return => {
                    let val = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    self.frames.pop();
                    if self.frames.is_empty() {
                        return Ok(val);
                    }
                    self.frames.last_mut().unwrap().stack.push(val);
                }
                Instr::Throw => {
                    let val = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    return Err(Ctrl::Throw(val));
                }
                Instr::Try(_target) => {}
                Instr::Catch => {}
                Instr::EndTry => {}
                Instr::TypeOf => {
                    let v = self.frames[frame_idx]
                        .stack
                        .pop()
                        .unwrap_or(Value::Undefined);
                    let t = match &v {
                        Value::Undefined => "undefined",
                        Value::Null => "object",
                        Value::Bool(_) => "boolean",
                        Value::Number(_) => "number",
                        Value::String(_) => "string",
                        Value::Function(_) => "function",
                        Value::Object(_) => "object",
                    };
                    self.frames[frame_idx]
                        .stack
                        .push(Value::String(t.to_string()));
                }
                Instr::Void => {
                    self.frames[frame_idx].stack.pop();
                    self.frames[frame_idx].stack.push(Value::Undefined);
                }
                Instr::New(_argc) => {
                    for _ in 0.._argc {
                        self.frames[frame_idx].stack.pop();
                    }
                    let _callee = self.frames[frame_idx].stack.pop();
                    let obj = self.heap.alloc_obj();
                    self.frames[frame_idx].stack.push(Value::Object(obj));
                }
                _ => {}
            }
        }
    }
}
