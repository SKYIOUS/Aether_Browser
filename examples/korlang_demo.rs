use korlang;

fn main() {
    let source = r#"
Component Demo {
    state counter: Int = 0
    fn increment(n) { n + 1 }
    Row {
        Text(text: "Counter: " + counter)
        for x in [1, 2, 3] {
            Text(text: "Item: " + x)
        }
    }
}
"#;
    let bytecode = korlang::compile(source);
    println!("Compiled Demo to {} opcodes", bytecode.len());
    let mut vm = korlang::VirtualMachine::new();
    vm.execute(bytecode);
}
