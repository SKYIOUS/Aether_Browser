use std::io::{self, Write};

fn main() {
    println!("Korlang REPL v0.2.0");
    println!("Type 'exit' to quit.");

    let mut vm = korlang::VirtualMachine::new();

    loop {
        print!("kor> ");
        io::stdout().flush().expect("Unexpected None value");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unexpected None value");
        let input = input.trim();

        if input == "exit" { break; }
        if input.is_empty() { continue; }

        // Wrap expression in a dummy component for compilation
        let source = format!("Component REPL {{ Row {{ Text(text: {}) }} }}", input);
        let bytecode = korlang::compile(&source);

        if bytecode.is_empty() {
            println!("Error: Could not compile expression");
            continue;
        }

        vm.execute(bytecode);

        if let Some(korlang::Value::Object(obj)) = vm.stack.last() {
            let obj = obj.lock().expect("Mutex poisoned");
            if let Some(korlang::Value::Object(text_obj)) = obj.children.first() {
                let text_obj = text_obj.lock().expect("Mutex poisoned");
                if let Some(val) = text_obj.properties.get("text") {
                    println!("=> {}", val.to_string_val());
                }
            }
        }

        vm.stack.clear();
    }
}
