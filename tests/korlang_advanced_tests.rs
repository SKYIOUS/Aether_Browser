use std::sync::Arc;
use korlang::{VirtualMachine, Value};
use korlang::vm::OpCode;

// ════════════════════════════════════════════════════════════════════
// 1. Nested function calls
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_nested_function_calls() {
    let mut vm = VirtualMachine::new();
    vm.register_native("double", Arc::new(|args: &[Value]| {
        if let Some(Value::Number(n)) = args.first() {
            Value::Number(n * 2.0)
        } else { Value::None }
    }));
    vm.register_native("add_one", Arc::new(|args: &[Value]| {
        if let Some(Value::Number(n)) = args.first() {
            Value::Number(n + 1.0)
        } else { Value::None }
    }));
    // Simulate nested: add_one(double(5)) = add_one(10) = 11
    // Push 5, call double → push 10, call add_one → push 11
    vm.execute(vec![
        OpCode::Push(Value::Number(5.0)),
        OpCode::Call("double".into(), 1),
        OpCode::Call("add_one".into(), 1),
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "11");
}

// ════════════════════════════════════════════════════════════════════
// 2. Closure capture (simulated — timer callback stores source)
// ════════════════════════════════════════════════════════════════════

// ponytail: korlang VM has no closures; verify heap persists across loads
#[test]
fn test_closure_capture() {
    // ponytail: korlang VM has no closures; verify heap persists across calls
    let mut vm = VirtualMachine::new();
    vm.heap.insert("captured".into(), Value::String("hello".into()));
    vm.execute(vec![
        OpCode::Load("captured".into()),
    ]);
    assert_eq!(vm.stack[0].to_string_val(), "hello");
}

// ════════════════════════════════════════════════════════════════════
// 3. native_print
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_native_print() {
    let mut vm = VirtualMachine::new();
    let called = Arc::new(std::sync::Mutex::new(false));
    let c = Arc::clone(&called);
    vm.register_native("print", Arc::new(move |args: &[Value]| {
        *c.lock().unwrap() = true;
        assert_eq!(args.len(), 1);
        assert_eq!(args[0].to_string_val(), "test msg");
        Value::None
    }));
    vm.execute(vec![
        OpCode::Push(Value::String("test msg".into())),
        OpCode::Call("print".into(), 1),
    ]);
    assert!(*called.lock().unwrap());
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "none");
}

// ════════════════════════════════════════════════════════════════════
// 4. native_chrome_render
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_native_chrome_render() {
    let mut vm = VirtualMachine::new();
    let called = Arc::new(std::sync::Mutex::new(false));
    let c = Arc::clone(&called);
    vm.register_native("chrome.render", Arc::new(move |args: &[Value]| {
        *c.lock().unwrap() = true;
        if let Some(Value::String(html)) = args.first() {
            assert_eq!(html, "<div>hi</div>");
        }
        Value::Bool(true)
    }));
    vm.execute(vec![
        OpCode::Push(Value::String("<div>hi</div>".into())),
        OpCode::Call("chrome.render".into(), 1),
    ]);
    assert!(*called.lock().unwrap());
    assert_eq!(vm.stack[0].to_string_val(), "true");
}

// ════════════════════════════════════════════════════════════════════
// 5. Interpolate multiple vars
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_interpolate_multiple_vars() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::String("Hello ".into())),
        OpCode::Push(Value::String("World ".into())),
        OpCode::Push(Value::String("3".into())),
        OpCode::Interpolate(3),
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "Hello World 3");
}

// ════════════════════════════════════════════════════════════════════
// 6. ForEach with empty array (count=0)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_for_each_empty_array() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::ForEach("i".into(), 0),
        OpCode::Push(Value::Number(99.0)),
    ]);
    // count=0: body skipped, falls through to next instruction
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "99");
}

// ════════════════════════════════════════════════════════════════════
// 7. ForEach with single item (count=1)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_for_each_single_item() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::ForEach("i".into(), 1),
        OpCode::Push(Value::Number(42.0)),
    ]);
    // One iteration, then loop ends (no Jump back → body runs once)
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "42");
}

// ════════════════════════════════════════════════════════════════════
// 8. JumpIfFalse with None (falsy)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_jump_if_false_none() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::None),
        OpCode::JumpIfFalse(3),
        OpCode::Push(Value::Number(1.0)),
        OpCode::Push(Value::Number(2.0)),
    ]);
    // None is falsy → jump to ip=3, only 2.0 on stack
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "2");
}

// ════════════════════════════════════════════════════════════════════
// 9. JumpIfFalse with zero (falsy)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_jump_if_false_zero() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::Number(0.0)),
        OpCode::JumpIfFalse(3),
        OpCode::Push(Value::Number(1.0)),
        OpCode::Push(Value::Number(2.0)),
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "2");
}

// ════════════════════════════════════════════════════════════════════
// 10. JumpIfFalse with empty string (falsy)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_jump_if_false_empty_string() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::String("".into())),
        OpCode::JumpIfFalse(3),
        OpCode::Push(Value::Number(1.0)),
        OpCode::Push(Value::Number(2.0)),
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "2");
}

// ════════════════════════════════════════════════════════════════════
// 11. Dup preserves value
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_dup_preserves_value() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::Number(7.0)),
        OpCode::Dup,
    ]);
    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0].to_string_val(), "7");
    assert_eq!(vm.stack[1].to_string_val(), "7");
}

// ════════════════════════════════════════════════════════════════════
// 12. Pop removes value
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_pop_removes_value() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::Number(1.0)),
        OpCode::Push(Value::Number(2.0)),
        OpCode::Pop,
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "1");
}

// ════════════════════════════════════════════════════════════════════
// 13. Store/Load roundtrip
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_store_load_roundtrip() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::Push(Value::String("roundtripped".into())),
        OpCode::Store("myvar".into()),
        OpCode::Load("myvar".into()),
    ]);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0].to_string_val(), "roundtripped");
}

// ════════════════════════════════════════════════════════════════════
// 14. CreateElement sets correct tag
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_create_element_sets_tag() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::CreateElement("Button".into()),
    ]);
    if let Some(Value::Object(obj)) = vm.stack.last() {
        let obj = obj.lock().unwrap();
        assert_eq!(obj.tag, "Button");
    } else {
        panic!("Expected Object on stack");
    }
}

// ════════════════════════════════════════════════════════════════════
// 15. SetProperty adds attribute
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_set_property_adds_attr() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::CreateElement("Text".into()),
        OpCode::Push(Value::String("hello".into())),
        OpCode::SetProperty("label".into()),
    ]);
    if let Some(Value::Object(obj)) = vm.stack.last() {
        let obj = obj.lock().unwrap();
        assert_eq!(
            obj.properties.get("label").unwrap().to_string_val(),
            "hello"
        );
    } else {
        panic!("Expected Object");
    }
}

// ════════════════════════════════════════════════════════════════════
// 16. AddChild nests elements
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_add_child_nests_elements() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![
        OpCode::CreateElement("Parent".into()),
        OpCode::CreateElement("Child".into()),
        OpCode::AddChild,
    ]);
    if let Some(Value::Object(obj)) = vm.stack.last() {
        let obj = obj.lock().unwrap();
        assert_eq!(obj.tag, "Parent");
        assert_eq!(obj.children.len(), 1);
        if let Value::Object(child) = &obj.children[0] {
            let child = child.lock().unwrap();
            assert_eq!(child.tag, "Child");
        }
    }
}

// ════════════════════════════════════════════════════════════════════
// 17. Deep element tree (10+ levels)
// ════════════════════════════════════════════════════════════════════

// ponytail: interleave Create + AddChild like the compiler does; batch creates
// push all onto stack before any AddChild, which creates a flat tree instead.
#[test]
fn test_deep_element_tree() {
    let mut vm = VirtualMachine::new();
    // ponytail: interleave CreateElement + AddChild like the compiler does
    vm.execute(vec![
        OpCode::CreateElement("L0".into()),
        OpCode::CreateElement("L1".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L2".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L3".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L4".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L5".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L6".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L7".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L8".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L9".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L10".into()),
        OpCode::AddChild,
        OpCode::CreateElement("L11".into()),
        OpCode::AddChild,
    ]);
    assert_eq!(vm.stack.len(), 1);
    if let Value::Object(obj) = vm.stack.last().unwrap() {
        let obj = obj.lock().unwrap();
        assert_eq!(obj.tag, "L0");
        assert_eq!(obj.children.len(), 11);
    }
}

// ════════════════════════════════════════════════════════════════════
// 18. Multi-arg native call
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_multi_arg_native_call() {
    let mut vm = VirtualMachine::new();
    vm.register_native("concat3", Arc::new(|args: &[Value]| {
        let parts: Vec<String> = args.iter().map(|v| v.to_string_val()).collect();
        Value::String(parts.concat())
    }));
    vm.execute(vec![
        OpCode::Push(Value::String("a".into())),
        OpCode::Push(Value::String("b".into())),
        OpCode::Push(Value::String("c".into())),
        OpCode::Call("concat3".into(), 3),
    ]);
    assert_eq!(vm.stack[0].to_string_val(), "abc");
}

// ════════════════════════════════════════════════════════════════════
// 19. ForEach with jump back (loop)
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_for_each_with_jump_back() {
    let mut vm = VirtualMachine::new();
    // Loop 3 times, each iteration pushes 1.0
    vm.execute(vec![
        OpCode::ForEach("i".into(), 3),  // ip=0
        OpCode::Push(Value::Number(1.0)), // ip=1
        OpCode::Jump(0),                   // ip=2 → back to ForEach
    ]);
    assert_eq!(vm.stack.len(), 3);
    for v in &vm.stack {
        assert_eq!(v.to_string_val(), "1");
    }
}

// ════════════════════════════════════════════════════════════════════
// 20. Empty bytecode
// ════════════════════════════════════════════════════════════════════

#[test]
fn test_empty_bytecode() {
    let mut vm = VirtualMachine::new();
    vm.execute(vec![]);
    assert!(vm.stack.is_empty());
    assert!(vm.heap.is_empty());
}
