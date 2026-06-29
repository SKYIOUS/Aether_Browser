pub mod compiler;
pub mod vm;
pub mod interop;
pub use compiler::compile;
pub use vm::{VirtualMachine, Value};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_korlang_basics() {
        assert!(true);
    }
}
