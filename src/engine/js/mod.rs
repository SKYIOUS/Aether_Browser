use rquickjs::{Context, Runtime as QuickJSRuntime};

pub struct JSEngine {
    runtime: QuickJSRuntime,
}

impl JSEngine {
    pub fn new() -> Self {
        let runtime = QuickJSRuntime::new().unwrap();
        Self { runtime }
    }

    pub fn execute(&mut self, code: &str) -> Result<String, String> {
        let ctx = Context::full(&self.runtime).unwrap();
        ctx.with(|ctx| {
            match ctx.eval::<String, _>(code) {
                Ok(result) => Ok(result),
                Err(e) => Err(format!("JS Error: {:?}", e)),
            }
        })
    }
}

pub struct Runtime {
    pub runtime: QuickJSRuntime,
}

impl Runtime {
    pub fn new() -> Self {
        let runtime = QuickJSRuntime::new().unwrap();
        Self { runtime }
    }

    pub fn execute(&self, code: &str) {
        let ctx = Context::full(&self.runtime).unwrap();
        ctx.with(|ctx| {
            let _: rquickjs::Value = ctx.eval(code).unwrap();
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_init() {
        let _runtime = Runtime::new();
    }
}