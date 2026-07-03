//! Two-phase compiler: lexer → tokens → parser → AST → bytecode.
//! The public entry point is [`compile()`] which takes Korlang source text
//! and returns a `Vec<OpCode>` consumable by [`VirtualMachine`](crate::vm::VirtualMachine).

pub mod lexer; pub mod parser;
use lexer::Lexer;
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

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);
fn next_label() -> usize {
    LABEL_COUNTER.fetch_add(1, Ordering::Relaxed)
}

fn emit_component(comp: &parser::Component, ops: &mut Vec<OpCode>) {
    for state in &comp.states { emit_expr(&state.default_value, ops); ops.push(OpCode::Store(state.name.clone())); }
    emit_node(&comp.root, ops);
}

fn emit_node(node: &Node, ops: &mut Vec<OpCode>) {
    match node {
        Node::Element(el) => emit_element(el, ops),
        Node::IfElse { condition, then_branch, else_branch } => emit_if_else(condition, then_branch, else_branch, ops),
        Node::ForLoop { var, count, body } => emit_for_loop(var, count, body, ops),
    }
}

fn emit_if_else(condition: &Expr, then_branch: &[Node], else_branch: &[Node], ops: &mut Vec<OpCode>) {
    let _label_id = next_label();
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

fn emit_for_loop(var: &str, count: &Expr, body: &[Node], ops: &mut Vec<OpCode>) {
    match count {
        Expr::Number(n) => {
            let c = *n as usize;
            let for_ip = ops.len();
            ops.push(OpCode::ForEach(var.to_string(), c));
            for child in body { emit_node(child, ops); }
            ops.push(OpCode::Jump(for_ip));
        }
        _ => {
            let c = 0;
            let for_ip = ops.len();
            ops.push(OpCode::ForEach(var.to_string(), c));
            for child in body { emit_node(child, ops); }
            ops.push(OpCode::Jump(for_ip));
        }
    }
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
    }
}
