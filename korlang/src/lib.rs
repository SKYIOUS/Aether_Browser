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
pub use vm::{VirtualMachine, Value, KorObject, NativeFn};

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

    // ── OpCode-level tests ──────────────────────────────────────────

    use crate::vm::OpCode;

    #[test]
    fn test_op_push() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(42.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack.last().unwrap().to_string_val(), "42");
    }

    #[test]
    fn test_op_load() {
        let mut vm = VirtualMachine::new();
        vm.heap.insert("x".into(), Value::String("val".into()));
        vm.execute(vec![
            OpCode::Load("x".into()),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack.last().unwrap().to_string_val(), "val");
    }

    #[test]
    fn test_op_load_missing() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Load("nonexistent".into()),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack.last().unwrap().to_string_val(), "none");
    }

    #[test]
    fn test_op_store() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(7.0)),
            OpCode::Store("key".into()),
        ]);
        assert!(vm.stack.is_empty());
        assert_eq!(vm.heap.get("key").unwrap().to_string_val(), "7");
    }

    #[test]
    fn test_op_create_element() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::CreateElement("Button".into()),
        ]);
        assert_eq!(vm.stack.len(), 1);
        if let Value::Object(obj) = vm.stack.last().unwrap() {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.tag, "Button");
            assert!(obj.properties.is_empty());
            assert!(obj.children.is_empty());
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_op_set_property() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::CreateElement("Text".into()),
            OpCode::Push(Value::String("hello".into())),
            OpCode::SetProperty("text".into()),
        ]);
        if let Some(Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().unwrap();
            assert_eq!(
                obj.properties.get("text").unwrap().to_string_val(),
                "hello"
            );
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_op_set_property_underflow() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::SetProperty("name".into()),
        ]);
        // should not panic, just log error
    }

    #[test]
    fn test_op_add_child() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::CreateElement("Parent".into()),
            OpCode::CreateElement("Child".into()),
            OpCode::AddChild,
        ]);
        assert_eq!(vm.stack.len(), 1);
        if let Value::Object(obj) = vm.stack.last().unwrap() {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.tag, "Parent");
            assert_eq!(obj.children.len(), 1);
        } else {
            panic!("Expected Object");
        }
    }

    #[test]
    fn test_op_add_child_underflow() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::AddChild,
        ]);
        // should not panic
    }

    #[test]
    fn test_op_dup() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(3.0)),
            OpCode::Dup,
        ]);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0].to_string_val(), "3");
        assert_eq!(vm.stack[1].to_string_val(), "3");
    }

    #[test]
    fn test_op_dup_underflow() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Dup,
        ]);
        // should not panic
    }

    #[test]
    fn test_op_pop() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(1.0)),
            OpCode::Pop,
        ]);
        assert!(vm.stack.is_empty());
    }

    #[test]
    fn test_op_pop_underflow() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Pop,
        ]);
        // should not panic
    }

    #[test]
    fn test_op_jump() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(1.0)),
            OpCode::Jump(3),       // skip Push(2)
            OpCode::Push(Value::Number(2.0)),
            OpCode::Push(Value::Number(3.0)),
        ]);
        // Should have 1 then 3 (skipped 2)
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0].to_string_val(), "1");
        assert_eq!(vm.stack[1].to_string_val(), "3");
    }

    #[test]
    fn test_op_jump_out_of_bounds() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Jump(999),
            OpCode::Push(Value::Number(1.0)),
        ]);
        // Should continue past invalid jump target
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn test_op_jump_if_false_true_condition() {
        let mut vm = VirtualMachine::new();
        // 4 instructions: Push(..)@0, JumpIfFalse@1, Push(..)@2, Push(..)@3
        // target 3 = skip Push(1.0), go to Push(2.0)
        vm.execute(vec![
            OpCode::Push(Value::Bool(true)),
            OpCode::JumpIfFalse(3),  // condition true → don't jump, fall through to ip=2
            OpCode::Push(Value::Number(1.0)),
            OpCode::Push(Value::Number(2.0)),
        ]);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0].to_string_val(), "1");
        assert_eq!(vm.stack[1].to_string_val(), "2");
    }

    #[test]
    fn test_op_jump_if_false_false_condition() {
        let mut vm = VirtualMachine::new();
        // target 3 = skip Push(1.0), jump to Push(2.0)
        vm.execute(vec![
            OpCode::Push(Value::Bool(false)),
            OpCode::JumpIfFalse(3),  // condition false → jump to ip=3
            OpCode::Push(Value::Number(1.0)),
            OpCode::Push(Value::Number(2.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "2");
    }

    #[test]
    fn test_op_jump_if_false_none_condition() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::None),
            OpCode::JumpIfFalse(3),  // None is falsy → jump to ip=3
            OpCode::Push(Value::Number(1.0)),
            OpCode::Push(Value::Number(2.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "2");
    }

    #[test]
    fn test_op_jump_if_false_zero_number() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(0.0)),
            OpCode::JumpIfFalse(3),  // 0 is falsy → jump to ip=3
            OpCode::Push(Value::Number(1.0)),
            OpCode::Push(Value::Number(2.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "2");
    }

    #[test]
    fn test_op_jump_if_false_empty_string() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::String("".into())),
            OpCode::JumpIfFalse(3),  // "" is falsy → jump to ip=3
            OpCode::Push(Value::Number(1.0)),
            OpCode::Push(Value::Number(2.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "2");
    }

    #[test]
    fn test_op_jump_if_false_out_of_bounds() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Bool(false)),
            OpCode::JumpIfFalse(999),
        ]);
        // should not panic
    }

    #[test]
    fn test_op_label() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Label("start".into()),
            OpCode::Push(Value::Number(1.0)),
        ]);
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn test_op_call_native() {
        use std::sync::Arc;
        let mut vm = VirtualMachine::new();
        vm.register_native("double", Arc::new(|args: &[Value]| {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n * 2.0)
            } else {
                Value::None
            }
        }));
        vm.execute(vec![
            OpCode::Push(Value::Number(21.0)),
            OpCode::Call("double".into(), 1),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "42");
    }

    #[test]
    fn test_op_call_no_such_native() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::Number(1.0)),
            OpCode::Call("nonexistent".into(), 1),
        ]);
        // Should push None as default
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "none");
    }

    #[test]
    fn test_op_call_args_order() {
        use std::sync::Arc;
        let mut vm = VirtualMachine::new();
        vm.register_native("concat", Arc::new(|args: &[Value]| {
            let strs: Vec<String> = args.iter().map(|v| v.to_string_val()).collect();
            Value::String(strs.concat())
        }));
        vm.execute(vec![
            OpCode::Push(Value::String("a".into())),
            OpCode::Push(Value::String("b".into())),
            OpCode::Push(Value::String("c".into())),
            OpCode::Call("concat".into(), 3),
        ]);
        // Args should be in order a, b, c (not c, b, a)
        assert_eq!(vm.stack[0].to_string_val(), "abc");
    }

    #[test]
    fn test_op_interpolate() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::Push(Value::String("Hello ".into())),
            OpCode::Push(Value::String("World".into())),
            OpCode::Interpolate(2),
        ]);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0].to_string_val(), "Hello World");
    }

    #[test]
    fn test_op_for_each() {
        let mut vm = VirtualMachine::new();
        // ForEach with Jump back runs iterations; without Jump back runs once
        vm.execute(vec![
            OpCode::ForEach("i".into(), 3),
            OpCode::Dup,
        ]);
        // First iteration stores i=0, Dup copies top of stack.
        // Stack was empty before, so Dup errors and continues.
        // But heap has __fe_i = 1 (iteration counter incremented)
        assert_eq!(vm.heap.get("__fe_i").unwrap().to_string_val(), "1");
    }

    #[test]
    fn test_op_for_each_zero() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::ForEach("i".into(), 0),
            OpCode::Push(Value::Number(1.0)),
        ]);
        // count=0: ForEach skips to else branch, falls through to ip+1
        // (end_key not set since body never ran, so ip+1 executes Push)
        assert_eq!(vm.stack.len(), 1);
    }

    #[test]
    fn test_op_for_each_with_jump_back() {
        let mut vm = VirtualMachine::new();
        // Proper loop: ForEach scans for Jump(0) to find loop end
        vm.execute(vec![
            OpCode::ForEach("i".into(), 2), // ip=0
            OpCode::Push(Value::Number(1.0)),
            OpCode::Jump(0),  // jump back to ForEach
        ]);
        // 2 iterations, each pushing 1
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0].to_string_val(), "1");
        assert_eq!(vm.stack[1].to_string_val(), "1");
    }

    // ── Edge cases ──────────────────────────────────────────────────

    #[test]
    fn test_empty_bytecode() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![]); // should not panic
    }

    #[test]
    fn test_stack_underflow_does_not_panic() {
        let mut vm = VirtualMachine::new();
        // Various underflow scenarios
        vm.execute(vec![
            OpCode::Pop,
            OpCode::Dup,
            OpCode::AddChild,
            OpCode::SetProperty("x".into()),
        ]);
    }

    #[test]
    fn test_type_mismatch_jump_if_false_with_object() {
        let mut vm = VirtualMachine::new();
        // Objects are truthy → JumpIfFalse doesn't jump; falls through
        vm.execute(vec![
            OpCode::CreateElement("Div".into()),  // push object on stack
            OpCode::JumpIfFalse(3),               // object is truthy → don't jump
            OpCode::Push(Value::Number(1.0)),
        ]);
        assert_eq!(vm.stack.len(), 2);  // object + 1.0
    }

    #[test]
    fn test_deep_element_tree() {
        let mut vm = VirtualMachine::new();
        vm.execute(vec![
            OpCode::CreateElement("A".into()),
            OpCode::CreateElement("B".into()),
            OpCode::AddChild,
            OpCode::CreateElement("C".into()),
            OpCode::AddChild,
        ]);
        // Stack: [A]
        // A has children [B, C]
        assert_eq!(vm.stack.len(), 1);
        if let Value::Object(obj) = vm.stack.last().unwrap() {
            let obj = obj.lock().unwrap();
            assert_eq!(obj.tag, "A");
            assert_eq!(obj.children.len(), 2);
        }
    }

    #[test]
    fn test_native_callback_print() {
        use std::sync::Arc;
        let mut vm = VirtualMachine::new();
        let called = Arc::new(std::sync::Mutex::new(false));
        let called_clone = Arc::clone(&called);
        vm.register_native("print", Arc::new(move |args: &[Value]| {
            *called_clone.lock().unwrap() = true;
            assert!(!args.is_empty());
            Value::None
        }));
        vm.execute(vec![
            OpCode::Push(Value::String("test".into())),
            OpCode::Call("print".into(), 1),
        ]);
        assert!(*called.lock().unwrap());
    }

    #[test]
    fn test_multi_arg_native() {
        use std::sync::Arc;
        let mut vm = VirtualMachine::new();
        vm.register_native("sum", Arc::new(|args: &[Value]| {
            let total: f64 = args.iter().map(|v| {
                if let Value::Number(n) = v { *n } else { 0.0 }
            }).sum();
            Value::Number(total)
        }));
        vm.execute(vec![
            OpCode::Push(Value::Number(10.0)),
            OpCode::Push(Value::Number(20.0)),
            OpCode::Push(Value::Number(30.0)),
            OpCode::Call("sum".into(), 3),
        ]);
        assert_eq!(vm.stack[0].to_string_val(), "60");
    }
}
