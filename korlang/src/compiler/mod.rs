//! Two-phase compiler: lexer → tokens → parser → AST → bytecode.
//! The public entry point is compile() which takes Korlang source text
//! and returns a Vec<OpCode> consumable by VirtualMachine.

pub mod lexer; pub mod parser; pub mod formatter;
use lexer::{Lexer, Token};
use parser::{Parser, Node, Element, Expr};
use crate::vm::{OpCode, Value};

pub fn compile(source: &str) -> Vec<OpCode> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let mut bytecode = Vec::new();
    if let Some(component) = parser.parse_component() { emit_component(&component, &mut bytecode); }
    bytecode
}

fn emit_component(comp: &parser::Component, ops: &mut Vec<OpCode>) {
    for state in &comp.states {
        emit_expr(&state.default_value, ops);
        ops.push(OpCode::Store(state.name.clone()));
    }
    for func in &comp.functions {
        let mut body_ops = Vec::new();
        for node in &func.body { emit_node(node, &mut body_ops); }
        ops.push(OpCode::StoreFn(func.name.clone(), func.params.clone(), body_ops));
    }
    emit_node(&comp.root, ops);
}

fn emit_node(node: &Node, ops: &mut Vec<OpCode>) {
    match node {
        Node::Element(el) => emit_element(el, ops),
        Node::IfElse { condition, then_branch, else_branch } => emit_if_else(condition, then_branch, else_branch, ops),
        Node::ForLoop { var, collection, body } => emit_for_loop(var, collection, body, ops),
    }
}

fn emit_if_else(condition: &Expr, then_branch: &[Node], else_branch: &[Node], ops: &mut Vec<OpCode>) {
    emit_expr(condition, ops);
    ops.push(OpCode::JumpIfFalse(0));
    let else_jump_idx = ops.len() - 1;
    for child in then_branch { emit_node(child, ops); }
    ops.push(OpCode::Jump(0));
    let end_jump_idx = ops.len() - 1;
    let else_ip = ops.len();
    ops[else_jump_idx] = OpCode::JumpIfFalse(else_ip);
    for child in else_branch { emit_node(child, ops); }
    let end_ip = ops.len();
    ops[end_jump_idx] = OpCode::Jump(end_ip);
}

fn emit_for_loop(var: &str, collection: &Expr, body: &[Node], ops: &mut Vec<OpCode>) {
    emit_expr(collection, ops);
    let for_ip = ops.len();
    ops.push(OpCode::ForEach(var.to_string(), 0));
    ops.push(OpCode::JumpIfFalse(0));
    let exit_jump_idx = ops.len() - 1;
    for child in body { emit_node(child, ops); }
    ops.push(OpCode::Jump(for_ip));
    let end_ip = ops.len();
    ops[exit_jump_idx] = OpCode::JumpIfFalse(end_ip);
}

fn emit_element(el: &Element, ops: &mut Vec<OpCode>) {
    ops.push(OpCode::CreateElement(el.name.clone()));
    for prop in &el.properties { emit_expr(&prop.value, ops); ops.push(OpCode::SetProperty(prop.name.clone())); }
    if let Some(ref h) = el.on_click { ops.push(OpCode::Push(Value::String(h.clone()))); ops.push(OpCode::SetProperty("on_click".to_string())); }
    for child in &el.children { emit_node(child, ops); ops.push(OpCode::AddChild); }
}

fn emit_expr(expr: &Expr, ops: &mut Vec<OpCode>) {
    match expr {
        Expr::Literal(s) => ops.push(OpCode::Push(Value::String(s.clone()))),
        Expr::Number(n) => ops.push(OpCode::Push(Value::Number(*n))),
        Expr::Identifier(id) | Expr::Binding(id) => ops.push(OpCode::Load(id.clone())),
        Expr::Call { name, args } => {
            let argc = args.len();
            for arg in args { emit_expr(arg, ops); }
            ops.push(OpCode::Call(name.clone(), argc));
        }
        Expr::List(items) => {
            let n = items.len();
            for item in items { emit_expr(item, ops); }
            ops.push(OpCode::MakeList(n));
        }
        Expr::Interpolated { parts, vars } => {
            for i in 0..vars.len() {
                ops.push(OpCode::Push(Value::String(parts[i].clone())));
                ops.push(OpCode::Load(vars[i].clone()));
            }
            if let Some(last) = parts.last() {
                ops.push(OpCode::Push(Value::String(last.clone())));
            }
            let total = parts.len() + vars.len();
            ops.push(OpCode::Interpolate(total));
        }
        Expr::BinaryOp { left, op, right } => {
            emit_expr(left, ops);
            emit_expr(right, ops);
            match op {
                Token::Plus => ops.push(OpCode::Add),
                Token::Minus => ops.push(OpCode::Sub),
                Token::Star => ops.push(OpCode::Mul),
                Token::Slash => ops.push(OpCode::Div),
                Token::And => ops.push(OpCode::And),
                Token::Or => ops.push(OpCode::Or),
                Token::Eq => ops.push(OpCode::Eq),
                Token::Neq => ops.push(OpCode::Neq),
                Token::Lt => ops.push(OpCode::Lt),
                Token::Gt => ops.push(OpCode::Gt),
                Token::Le => ops.push(OpCode::Le),
                Token::Ge => ops.push(OpCode::Ge),
                _ => {}
            }
        }
        Expr::UnaryOp { op, expr } => {
            emit_expr(expr, ops);
            match op {
                Token::Not => ops.push(OpCode::Not),
                Token::Minus => {
                    ops.push(OpCode::Push(Value::Number(-1.0)));
                    ops.push(OpCode::Mul);
                }
                _ => {}
            }
        }
    }
}
