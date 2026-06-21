pub mod lexer; pub mod parser; pub mod rust_gen;
use lexer::Lexer;
use parser::{Parser, Component, Element, Expr};
use crate::vm::{OpCode, Value};

pub fn compile(source: &str) -> Vec<OpCode> {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let mut bytecode = Vec::new();
    if let Some(component) = parser.parse_component() { emit_component(&component, &mut bytecode); }
    bytecode
}
fn emit_component(comp: &Component, ops: &mut Vec<OpCode>) {
    for state in &comp.states { emit_expr(&state.default_value, ops); ops.push(OpCode::Store(state.name.clone())); }
    emit_element(&comp.root, ops);
}
fn emit_element(el: &Element, ops: &mut Vec<OpCode>) {
    ops.push(OpCode::CreateElement(el.name.clone()));
    for prop in &el.properties { emit_expr(&prop.value, ops); ops.push(OpCode::SetProperty(prop.name.clone())); }
    if let Some(ref h) = el.on_click { ops.push(OpCode::Push(Value::String(h.clone()))); ops.push(OpCode::SetProperty("on_click".to_string())); }
    for child in &el.children { emit_element(child, ops); ops.push(OpCode::AddChild); }
}
fn emit_expr(expr: &Expr, ops: &mut Vec<OpCode>) {
    match expr {
        Expr::Literal(s) => ops.push(OpCode::Push(Value::String(s.clone()))),
        Expr::Number(n) => ops.push(OpCode::Push(Value::Number(*n))),
        Expr::Identifier(id) | Expr::Binding(id) => ops.push(OpCode::Load(id.clone())),
    }
}
