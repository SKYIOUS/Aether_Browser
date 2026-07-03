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
pub use vm::{VirtualMachine, Value, KorObject};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_and_run_simple_component() {
        let source = r#"
            Component Hello {
                Text(text: "world")
            }
        "#;
        let bytecode = compile(source);
        assert!(!bytecode.is_empty(), "should produce bytecode");

        let mut vm = VirtualMachine::new();
        vm.execute(bytecode);
        assert!(!vm.stack.is_empty(), "should have result on stack");
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.tag, "Text");
            assert_eq!(
                obj.properties.get("text").and_then(|v| {
                    if let Value::String(s) = v { Some(s.clone()) } else { None }
                }),
                Some("world".to_string())
            );
        } else {
            panic!("Expected Object on stack");
        }
    }

    #[test]
    fn test_compile_and_run_row() {
        let source = r#"
            Component NavBar {
                Row(spacing: 8) {
                    Button(text: "Back", on_click: "back")
                    Button(text: "Forward", on_click: "forward")
                }
            }
        "#;
        let bytecode = compile(source);
        assert!(!bytecode.is_empty());
        let mut vm = VirtualMachine::new();
        vm.execute(bytecode);
        let root = vm.stack.last().cloned().unwrap_or(Value::None);
        if let Value::Object(obj) = root {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.tag, "Row");
            assert_eq!(obj.children.len(), 2, "Row should have 2 children");
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_binding_and_store() {
        let source = r#"
            Component Counter {
                state count: Number = 0
                Text(text: bind count)
            }
        "#;
        let bytecode = compile(source);
        assert!(!bytecode.is_empty());
        let mut vm = VirtualMachine::new();
        vm.execute(bytecode);
        assert!(!vm.stack.is_empty());
    }

    #[test]
    fn test_update_state() {
        let source = r#"
            Component Dynamic {
                Text(text: bind label)
            }
        "#;
        let bytecode = compile(source);
        let mut vm = VirtualMachine::new();
        vm.heap.insert("label".into(), Value::String("initial".into()));
        vm.execute(bytecode);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            assert_eq!(
                obj.properties.get("text").and_then(|v| {
                    if let Value::String(s) = v { Some(s.clone()) } else { None }
                }),
                Some("initial".to_string())
            );
        } else {
            panic!("Expected Object");
        }
        vm.update_state("label", Value::String("updated".into()));
    }

    #[test]
    fn test_builtins() {
        let source = r#"
            Component WithBuiltin {
                Text(text: bind greeting)
            }
        "#;
        let bytecode = compile(source);
        let mut vm = VirtualMachine::new();
        vm.heap.insert("greeting".into(), Value::String("Hello".into()));
        vm.execute(bytecode);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            assert_eq!(
                obj.properties.get("text").and_then(|v| {
                    if let Value::String(s) = v { Some(s.clone()) } else { None }
                }),
                Some("Hello".to_string())
            );
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_empty_source_does_not_crash() {
        let bytecode = compile("");
        let mut vm = VirtualMachine::new();
        vm.execute(bytecode); // should not panic
    }

    #[test]
    fn test_value_conversions() {
        assert_eq!(Value::String("hi".into()).to_string_val(), "hi");
        assert_eq!(Value::Number(42.0).to_string_val(), "42");
        assert_eq!(Value::Bool(true).to_string_val(), "true");
        assert_eq!(Value::None.to_string_val(), "none");
    }
}
