//! Korlang — a minimal UI DSL compiled to stack-based bytecode.
//!
//! Inspired by SwiftUI / QML. Korlang components are defined declaratively
//! and compiled into [`OpCode`](vm::OpCode) instructions that a
//! [`VirtualMachine`](vm::VirtualMachine) evaluates to produce a tree of
//! [`KorObject`](vm::KorObject) nodes.
//!
//! # Example
//! ```
//! # use korlang::{compile, VirtualMachine};
//! let bc = compile("Component Hello { Text(text: \"world\") }");
//! let mut vm = VirtualMachine::new();
//! vm.execute(bc);
//! ```

pub mod compiler;
pub mod vm;
pub use compiler::compile;
pub use compiler::formatter::format_component;
pub use vm::{VirtualMachine, Value, KorObject, NativeFn};

#[cfg(test)]
#[cfg(test)]
mod overhaul_tests {
    use super::*;

    #[test]
    fn test_arithmetic() {
        let code = "Component T { Row { Text(text: 10 + 5 * 2) } }";
        let bc = compile(code);
        let mut vm = VirtualMachine::new();
        vm.execute(bc);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            if let Some(Value::Object(text_obj)) = obj.children.first() {
                let text_obj = text_obj.lock().unwrap();
                assert_eq!(text_obj.properties.get("text").unwrap().to_string_val(), "20");
            }
        }
    }

    #[test]
    fn test_functions() {
        let code = "Component T { fn add(a, b) { a + b } Row { Text(text: add(10, 20)) } }";
        let bc = compile(code);
        let mut vm = VirtualMachine::new();
        vm.execute(bc);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            if let Some(Value::Object(text_obj)) = obj.children.first() {
                let text_obj = text_obj.lock().unwrap();
                assert_eq!(text_obj.properties.get("text").unwrap().to_string_val(), "30");
            }
        }
    }

    #[test]
    fn test_list_iteration() {
        let code = "Component T { Row { for x in [1, 2, 3] { Text(text: x) } } }";
        let bc = compile(code);
        let mut vm = VirtualMachine::new();
        vm.execute(bc);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.children.len(), 3);
        }
    }

    #[test]
    fn test_formatter_roundtrip() {
        let code = "Component T {\n    state counter: Int = 0\n    fn add(a) {\n        (a + 1)\n    }\n    Row {\n        Text(text: \"Hi\")\n    }\n}\n";
        let mut lexer = compiler::lexer::Lexer::new(code);
        let mut parser = compiler::parser::Parser::new(lexer.tokenize());
        if let Some(comp) = parser.parse_component() {
            let formatted = format_component(&comp);
            assert!(formatted.contains("Component T"));
            assert!(formatted.contains("state counter: Int = 0"));
            assert!(formatted.contains("fn add(a)"));
        }
    }
}
