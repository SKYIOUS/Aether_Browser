use std::collections::HashMap;

use crate::value::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ObjId {
    pub id: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FunId {
    pub id: u32,
}

pub struct Heap {
    objects: Vec<Obj>,
    funs: Vec<Fun>,
    free_obj: Vec<u32>,
    free_fun: Vec<u32>,
    global: Option<ObjId>,
}

struct Obj {
    props: HashMap<String, Value>,
    marked: bool,
}

pub struct Fun {
    pub name: String,
    pub params: Vec<String>,
    pub instrs: Vec<u8>,
    pub constants: Vec<f64>,
    pub strings: Vec<String>,
    pub locals_count: u32,
    pub env: Vec<(String, Value)>,
    marked: bool,
}

impl Heap {
    pub fn new() -> Self {
        let mut h = Self {
            objects: Vec::new(),
            funs: Vec::new(),
            free_obj: Vec::new(),
            free_fun: Vec::new(),
            global: None,
        };
        h.global = Some(h.alloc_obj());
        h
    }

    pub fn global(&self) -> ObjId {
        self.global.unwrap()
    }

    pub fn alloc_obj(&mut self) -> ObjId {
        if let Some(id) = self.free_obj.pop() {
            self.objects[id as usize] = Obj {
                props: HashMap::new(),
                marked: false,
            };
            ObjId { id }
        } else {
            let id = self.objects.len() as u32;
            self.objects.push(Obj {
                props: HashMap::new(),
                marked: false,
            });
            ObjId { id }
        }
    }

    pub fn obj_get(&self, id: ObjId, key: &str) -> Option<&Value> {
        self.objects[id.id as usize].props.get(key)
    }

    pub fn obj_set(&mut self, id: ObjId, key: String, val: Value) {
        self.objects[id.id as usize].props.insert(key, val);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn alloc_fun(
        &mut self,
        name: String,
        params: Vec<String>,
        instrs: Vec<u8>,
        constants: Vec<f64>,
        strings: Vec<String>,
        locals_count: u32,
        env: Vec<(String, Value)>,
    ) -> FunId {
        if let Some(id) = self.free_fun.pop() {
            self.funs[id as usize] = Fun {
                name,
                params,
                instrs,
                constants,
                strings,
                locals_count,
                env,
                marked: false,
            };
            FunId { id }
        } else {
            let id = self.funs.len() as u32;
            self.funs.push(Fun {
                name,
                params,
                instrs,
                constants,
                strings,
                locals_count,
                env,
                marked: false,
            });
            FunId { id }
        }
    }

    pub fn fun(&self, id: FunId) -> &Fun {
        &self.funs[id.id as usize]
    }

    pub fn fun_mut(&mut self, id: FunId) -> &mut Fun {
        &mut self.funs[id.id as usize]
    }

    pub fn sweep(&mut self) {
        for obj in &mut self.objects {
            obj.marked = false;
        }
        for fun in &mut self.funs {
            fun.marked = false;
        }
        if let Some(gid) = self.global {
            self.mark_obj(gid);
        }
        for i in 0..self.objects.len() {
            if !self.objects[i].marked {
                self.free_obj.push(i as u32);
            }
        }
        for i in 0..self.funs.len() {
            if !self.funs[i].marked {
                self.free_fun.push(i as u32);
            }
        }
    }

    fn mark_obj(&mut self, id: ObjId) {
        if self.objects[id.id as usize].marked {
            return;
        }
        self.objects[id.id as usize].marked = true;
        let props: Vec<Value> = self.objects[id.id as usize]
            .props
            .values()
            .cloned()
            .collect();
        for v in props {
            self.mark_val(&v);
        }
    }

    fn mark_fun(&mut self, id: FunId) {
        if self.funs[id.id as usize].marked {
            return;
        }
        self.funs[id.id as usize].marked = true;
        let env: Vec<(String, Value)> = self.funs[id.id as usize].env.to_vec();
        for (_, v) in env {
            self.mark_val(&v);
        }
    }

    fn mark_val(&mut self, v: &Value) {
        match v {
            Value::Object(id) => self.mark_obj(*id),
            Value::Function(id) => self.mark_fun(*id),
            _ => {}
        }
    }
}

impl Default for Heap {
    fn default() -> Self {
        Self::new()
    }
}
