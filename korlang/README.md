# Korlang

A minimal UI DSL compiled to stack-based bytecode. Inspired by SwiftUI / QML.

## Example

```korlang
Component Counter {
    state count: Number = 0
    Text(text: bind count)
    Button(text: "Increment", on_click: "inc")
}
```

## Building

```bash
cargo build -p korlang
```

## Testing

```bash
cargo test -p korlang
```

## API

- `compile(source: &str) -> Vec<OpCode>` — parse & compile Korlang source
- `VirtualMachine::new()` — create a VM
- `vm.execute(bytecode)` — run bytecode, producing a tree on `vm.stack`
- `vm.update_state(name, value)` — mutate a heap variable after execution
- `vm.set_builtin(name, value)` — register a built-in function

## OpCodes

| OpCode | Effect |
|--------|--------|
| `Push(v)` | Push a value onto the stack |
| `Load(name)` | Push a heap variable onto the stack |
| `Store(name)` | Pop the stack into a heap variable |
| `CreateElement(tag)` | Create a `KorObject` on the stack |
| `SetProperty(name)` | Pop a value and set as a property on the top element |
| `AddChild` | Pop a child and append to the top element's children |
| `Jump(target)` | Unconditional jump |
| `JumpIfFalse(target)` | Pop condition; jump if false |
| `ForEach(var, count)` | Iterate `count` times with loop variable `var` |
| `Interpolate(n)` | Concatenate `n` stack values into a string |
| `Call(name, argc)` | Call a built-in |
| `Dup` | Duplicate the top of the stack |
| `Pop` | Discard the top of the stack |
