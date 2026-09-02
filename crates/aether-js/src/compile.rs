use crate::ast::*;
use crate::bytecode::*;

pub fn compile(stmts: &[Stmt]) -> Result<Function, CompileError> {
    let mut c = Compiler::new();
    let len = stmts.len();
    for (i, stmt) in stmts.iter().enumerate() {
        if i == len - 1 {
            c.compile_stmt_final(stmt)?;
        } else {
            c.compile_stmt(stmt)?;
        }
    }
    Ok(c.finish())
}

#[derive(Debug)]
pub struct CompileError {
    pub msg: String,
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "compile error: {}", self.msg)
    }
}

struct Compiler {
    func: Function,
    scopes: Vec<Vec<String>>,
}

impl Compiler {
    fn new() -> Self {
        Self {
            func: Function {
                name: "<main>".to_string(),
                params: Vec::new(),
                instrs: Vec::new(),
                constants: Vec::new(),
                strings: Vec::new(),
                locals: Vec::new(),
                is_closure: false,
            },
            scopes: vec![Vec::new()],
        }
    }

    fn finish(self) -> Function {
        self.func
    }

    fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &str) -> u32 {
        let slot = self.func.locals.len() as u32;
        self.func.locals.push(name.to_string());
        self.scopes.last_mut().unwrap().push(name.to_string());
        slot
    }

    fn resolve_local(&self, name: &str) -> Option<u32> {
        if self.scopes.iter().any(|s| s.iter().any(|n| n == name)) {
            self.func
                .locals
                .iter()
                .position(|n| n == name)
                .map(|p| p as u32)
        } else {
            None
        }
    }

    fn add_constant(&mut self, n: f64) -> u32 {
        let slot = self.func.constants.len() as u32;
        self.func.constants.push(n);
        slot
    }

    fn add_string(&mut self, s: &str) -> u32 {
        let slot = self.func.strings.len() as u32;
        self.func.strings.push(s.to_string());
        slot
    }

    fn emit(&mut self, instr: Instr) {
        self.func.instrs.push(instr);
    }

    fn emit_jump(&mut self, instr: Instr) -> usize {
        let pos = self.func.instrs.len();
        self.emit(instr);
        pos
    }

    fn patch_jump(&mut self, pos: usize) {
        let target = self.func.instrs.len();
        match &mut self.func.instrs[pos] {
            Instr::Jmp(ref mut t) | Instr::JmpIf(ref mut t) | Instr::JmpIfNot(ref mut t) => {
                *t = target;
            }
            Instr::Try(ref mut t) => {
                *t = target;
            }
            _ => {}
        }
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Empty => {}
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                self.emit(Instr::Pop);
            }
            Stmt::Block(stmts) => {
                self.push_scope();
                for s in stmts {
                    self.compile_stmt(s)?;
                }
                self.pop_scope();
            }
            Stmt::VarDecl { name, init } => {
                let slot = self.declare(name);
                if let Some(init) = init {
                    self.compile_expr(init)?;
                    self.emit(Instr::SetLocal(slot));
                }
            }
            Stmt::LetDecl { name, init } => {
                let slot = self.declare(name);
                if let Some(init) = init {
                    self.compile_expr(init)?;
                    self.emit(Instr::SetLocal(slot));
                }
            }
            Stmt::ConstDecl { name, init } => {
                let slot = self.declare(name);
                self.compile_expr(init)?;
                self.emit(Instr::SetLocal(slot));
            }
            Stmt::If {
                test,
                consequent,
                alternate,
            } => {
                self.compile_expr(test)?;
                let else_jmp = self.emit_jump(Instr::JmpIfNot(0));
                self.compile_stmt(consequent)?;
                if let Some(alt) = alternate {
                    let done_jmp = self.emit_jump(Instr::Jmp(0));
                    self.patch_jump(else_jmp);
                    self.compile_stmt(alt)?;
                    self.patch_jump(done_jmp);
                } else {
                    self.patch_jump(else_jmp);
                }
            }
            Stmt::While { test, body } => {
                let loop_start = self.func.instrs.len();
                self.compile_expr(test)?;
                let exit_jmp = self.emit_jump(Instr::JmpIfNot(0));
                self.push_scope();
                self.compile_stmt(body)?;
                self.pop_scope();
                self.emit(Instr::Jmp(loop_start));
                self.patch_jump(exit_jmp);
            }
            Stmt::For {
                init,
                test,
                update,
                body,
            } => {
                self.push_scope();
                if let Some(init) = init {
                    self.compile_stmt(init)?;
                }
                let loop_start = self.func.instrs.len();
                if let Some(test) = test {
                    self.compile_expr(test)?;
                    let exit_jmp = self.emit_jump(Instr::JmpIfNot(0));
                    self.push_scope();
                    self.compile_stmt(body)?;
                    self.pop_scope();
                    if let Some(update) = update {
                        self.compile_expr(update)?;
                        self.emit(Instr::Pop);
                    }
                    self.emit(Instr::Jmp(loop_start));
                    self.patch_jump(exit_jmp);
                } else {
                    self.push_scope();
                    self.compile_stmt(body)?;
                    self.pop_scope();
                    if let Some(update) = update {
                        self.compile_expr(update)?;
                        self.emit(Instr::Pop);
                    }
                    self.emit(Instr::Jmp(loop_start));
                }
                self.pop_scope();
            }
            Stmt::Return(val) => {
                if let Some(v) = val {
                    self.compile_expr(v)?;
                } else {
                    self.emit(Instr::Undefined);
                }
                self.emit(Instr::Return);
            }
            Stmt::Throw(expr) => {
                self.compile_expr(expr)?;
                self.emit(Instr::Throw);
            }
            Stmt::Try {
                body,
                catch,
                finally,
            } => {
                let try_jmp = self.emit_jump(Instr::Try(0));
                self.compile_stmt(body)?;
                self.emit(Instr::EndTry);
                if let Some((name, catch_body)) = catch {
                    let skip_catch = self.emit_jump(Instr::Jmp(0));
                    self.patch_jump(try_jmp);
                    self.emit(Instr::Catch);
                    let slot = self.declare(name);
                    self.emit(Instr::SetLocal(slot));
                    self.compile_stmt(catch_body)?;
                    self.patch_jump(skip_catch);
                } else {
                    self.patch_jump(try_jmp);
                }
                if let Some(fin) = finally {
                    self.compile_stmt(fin)?;
                }
            }
            Stmt::FunctionDecl { name, params, body } => {
                let mut inner = Compiler::new();
                inner.func.name = name.clone();
                inner.func.params = params.clone();
                inner.push_scope();
                for p in params {
                    inner.declare(p);
                }
                if let Stmt::Block(stmts) = body.as_ref() {
                    for s in stmts {
                        inner.compile_stmt(s)?;
                    }
                }
                inner.pop_scope();
                let fun_idx = self.add_string(name);
                self.emit(Instr::MakeClosure(fun_idx));
                let slot = self.declare(name);
                self.emit(Instr::SetLocal(slot));
                let _ = inner;
            }
            _ => {}
        }
        Ok(())
    }

    fn compile_stmt_final(&mut self, stmt: &Stmt) -> Result<(), CompileError> {
        match stmt {
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                Ok(())
            }
            _ => self.compile_stmt(stmt),
        }
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), CompileError> {
        match expr {
            Expr::Literal(lit) => {
                self.compile_literal(lit);
                Ok(())
            }
            Expr::Identifier(name) => {
                if let Some(slot) = self.resolve_local(name) {
                    self.emit(Instr::GetLocal(slot));
                } else {
                    let s = self.add_string(name);
                    self.emit(Instr::GetGlobal(s));
                }
                Ok(())
            }
            Expr::This => {
                self.emit(Instr::Undefined);
                Ok(())
            }
            Expr::Binary { op, left, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                match op {
                    BinOp::Add => self.emit(Instr::Add),
                    BinOp::Sub => self.emit(Instr::Sub),
                    BinOp::Mul => self.emit(Instr::Mul),
                    BinOp::Div => self.emit(Instr::Div),
                    BinOp::Mod => self.emit(Instr::Mod),
                    BinOp::Pow => {
                        self.emit(Instr::Mul);
                    }
                    BinOp::Eq => self.emit(Instr::Eq),
                    BinOp::StrictEq => self.emit(Instr::StrictEq),
                    BinOp::Ne => self.emit(Instr::Ne),
                    BinOp::StrictNe => self.emit(Instr::StrictNe),
                    BinOp::Lt => self.emit(Instr::Lt),
                    BinOp::Gt => self.emit(Instr::Gt),
                    BinOp::LtEq => self.emit(Instr::LtEq),
                    BinOp::GtEq => self.emit(Instr::GtEq),
                    BinOp::And => {
                        self.emit(Instr::Pop);
                    }
                    BinOp::Or => {
                        self.emit(Instr::Pop);
                    }
                    _ => {
                        self.emit(Instr::Add);
                    }
                }
                Ok(())
            }
            Expr::Unary {
                op,
                operand,
                prefix: _,
            } => {
                self.compile_expr(operand)?;
                match op {
                    UnaryOp::Neg => self.emit(Instr::Neg),
                    UnaryOp::Not => self.emit(Instr::Not),
                    UnaryOp::BitNot => self.emit(Instr::BitNot),
                    UnaryOp::TypeOf => self.emit(Instr::TypeOf),
                    UnaryOp::Void => {
                        self.emit(Instr::Pop);
                        self.emit(Instr::Undefined);
                    }
                    _ => {}
                }
                Ok(())
            }
            Expr::Assignment { op, target, value } => {
                self.compile_expr(value)?;
                match target.as_ref() {
                    Expr::Identifier(name) => {
                        if let Some(slot) = self.resolve_local(name) {
                            if *op != AssignOp::Assign {
                                self.emit(Instr::GetLocal(slot));
                                self.emit(Instr::Add);
                            }
                            self.emit(Instr::SetLocal(slot));
                        } else {
                            let s = self.add_string(name);
                            if *op != AssignOp::Assign {
                                self.emit(Instr::GetGlobal(s));
                                self.emit(Instr::Add);
                            }
                            self.emit(Instr::SetGlobal(s));
                        }
                    }
                    _ => {
                        self.emit(Instr::Pop);
                    }
                }
                Ok(())
            }
            Expr::Call { callee, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.compile_expr(callee)?;
                self.emit(Instr::Call(args.len() as u32));
                Ok(())
            }
            Expr::FunctionExpr { name, params, body } => {
                let mut inner = Compiler::new();
                if let Some(n) = name {
                    inner.func.name = n.clone();
                }
                inner.func.params = params.clone();
                inner.push_scope();
                for p in params {
                    inner.declare(p);
                }
                if let Stmt::Block(stmts) = body.as_ref() {
                    for s in stmts {
                        inner.compile_stmt(s)?;
                    }
                }
                inner.pop_scope();
                let name_str = self.add_string(name.as_deref().unwrap_or("<anon>"));
                self.emit(Instr::MakeClosure(name_str));
                let _ = inner;
                Ok(())
            }
            Expr::Conditional {
                test,
                consequent,
                alternate,
            } => {
                self.compile_expr(test)?;
                let else_jmp = self.emit_jump(Instr::JmpIfNot(0));
                self.compile_expr(consequent)?;
                let done_jmp = self.emit_jump(Instr::Jmp(0));
                self.patch_jump(else_jmp);
                self.compile_expr(alternate)?;
                self.patch_jump(done_jmp);
                Ok(())
            }
            Expr::New { callee, args } => {
                for arg in args {
                    self.compile_expr(arg)?;
                }
                self.compile_expr(callee)?;
                self.emit(Instr::New(args.len() as u32));
                Ok(())
            }
            Expr::Member {
                object,
                property,
                computed,
            } => {
                self.compile_expr(object)?;
                if *computed {
                    self.compile_expr(property)?;
                } else if let Expr::Identifier(name) = property.as_ref() {
                    let s = self.add_string(name);
                    self.emit(Instr::ConstString(s));
                }
                Ok(())
            }
        }
    }

    fn compile_literal(&mut self, lit: &Lit) {
        match lit {
            Lit::Number(n) => {
                let slot = self.add_constant(*n);
                self.emit(Instr::Const(slot));
            }
            Lit::String(s) => {
                let slot = self.add_string(s);
                self.emit(Instr::ConstString(slot));
            }
            Lit::Bool(b) => {
                self.emit(Instr::ConstBool(if *b { 1 } else { 0 }));
            }
            Lit::Null => self.emit(Instr::Null),
            Lit::Undefined => self.emit(Instr::Undefined),
        }
    }
}
