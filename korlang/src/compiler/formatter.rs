use super::parser::{Component, Node, Expr, FunctionDef};
use super::lexer::Token;

pub fn format_component(comp: &Component) -> String {
    let mut out = format!("Component {} {{\n", comp.name);
    for state in &comp.states {
        out.push_str(&format!("    state {}: {} = {}\n", state.name, state.type_name, format_expr(&state.default_value)));
    }
    for func in &comp.functions {
        out.push_str(&format_function(func, 1));
    }
    out.push_str(&format_node(&comp.root, 1));
    out.push_str("}\n");
    out
}

fn format_function(func: &FunctionDef, indent: usize) -> String {
    let mut out = format!("{}fn {}({}) {{\n", "    ".repeat(indent), func.name, func.params.join(", "));
    for node in &func.body {
        out.push_str(&format_node(node, indent + 1));
    }
    out.push_str(&format!("{}}}\n", "    ".repeat(indent)));
    out
}

fn format_node(node: &Node, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    match node {
        Node::Element(el) => {
            let mut out = format!("{}{}", ind, el.name);
            if !el.properties.is_empty() {
                out.push('(');
                let props: Vec<String> = el.properties.iter().map(|p| format!("{}: {}", p.name, format_expr(&p.value))).collect();
                out.push_str(&props.join(", "));
                out.push(')');
            }
            if !el.children.is_empty() || el.on_click.is_some() {
                out.push_str(" {\n");
                if let Some(ref h) = el.on_click {
                    out.push_str(&format!("{}    on_click: \"{}\"\n", ind, h));
                }
                for child in &el.children {
                    out.push_str(&format_node(child, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
            } else {
                out.push('\n');
            }
            out
        }
        Node::IfElse { condition, then_branch, else_branch } => {
            let mut out = format!("{}if {} {{\n", ind, format_expr(condition));
            for child in then_branch { out.push_str(&format_node(child, indent + 1)); }
            out.push_str(&format!("{}}}", ind));
            if !else_branch.is_empty() {
                out.push_str(" else {\n");
                for child in else_branch { out.push_str(&format_node(child, indent + 1)); }
                out.push_str(&format!("{}}}\n", ind));
            } else {
                out.push('\n');
            }
            out
        }
        Node::ForLoop { var, collection, body } => {
            let mut out = format!("{}for {} in {} {{\n", ind, var, format_expr(collection));
            for child in body { out.push_str(&format_node(child, indent + 1)); }
            out.push_str(&format!("{}}}\n", ind));
            out
        }
    }
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal(s) => format!("\"{}\"", s),
        Expr::Number(n) => n.to_string(),
        Expr::Identifier(id) => id.clone(),
        Expr::Binding(id) => format!("bind {}", id),
        Expr::Call { name, args } => {
            let formatted_args: Vec<String> = args.iter().map(format_expr).collect();
            format!("{}({})", name, formatted_args.join(", "))
        }
        Expr::List(items) => {
            let formatted_items: Vec<String> = items.iter().map(format_expr).collect();
            format!("[{}]", formatted_items.join(", "))
        }
        Expr::Interpolated { parts, vars } => {
            let mut out = "\"".to_string();
            for i in 0..vars.len() {
                out.push_str(&parts[i]);
                out.push('$');
                out.push_str(&vars[i]);
            }
            if let Some(last) = parts.last() { out.push_str(last); }
            out.push('\"');
            out
        }
        Expr::BinaryOp { left, op, right } => {
            format!("({} {} {})", format_expr(left), format_token(op), format_expr(right))
        }
        Expr::UnaryOp { op, expr } => {
            format!("{}{}", format_token(op), format_expr(expr))
        }
    }
}

fn format_token(tok: &Token) -> &'static str {
    match tok {
        Token::Plus => "+",
        Token::Minus => "-",
        Token::Star => "*",
        Token::Slash => "/",
        Token::And => "&&",
        Token::Or => "||",
        Token::Not => "!",
        Token::Eq => "==",
        Token::Neq => "!=",
        Token::Lt => "<",
        Token::Gt => ">",
        Token::Le => "<=",
        Token::Ge => ">=",
        _ => "",
    }
}
