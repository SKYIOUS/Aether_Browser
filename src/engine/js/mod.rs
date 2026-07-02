pub mod js_bridge;

use std::sync::{Arc, Mutex};
use rquickjs::{Context, Runtime as QuickJSRuntime};

pub use js_bridge::JsBridge;

pub struct JSEngine {
    context: Option<rquickjs::Context>,
    #[allow(dead_code)]
    runtime: QuickJSRuntime,
}

impl JSEngine {
    pub fn new() -> Self {
        let runtime = QuickJSRuntime::new().unwrap();
        let ctx = rquickjs::Context::full(&runtime).unwrap();
        Self {
            runtime,
            context: Some(ctx),
        }
    }
}

impl Default for JSEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl JSEngine {
    pub fn execute(&mut self, code: &str) -> Result<String, String> {
        if let Some(ref ctx) = self.context {
            ctx.with(|ctx: rquickjs::Ctx<'_>| {
                match ctx.eval::<String, _>(code) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("JS Error: {:?}", e)),
                }
            })
        } else {
            Err("No JS context".to_string())
        }
    }

    pub fn execute_with_bridge(&mut self, code: &str, bridge: &Arc<Mutex<JsBridge>>) -> Result<String, String> {
        if let Some(ref ctx) = self.context {
            ctx.with(|ctx: rquickjs::Ctx<'_>| {
                let _ = js_bridge::register_browser_api(&ctx, bridge);
                match ctx.eval::<String, _>(code) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(format!("JS Error: {:?}", e)),
                }
            })
        } else {
            Err("No JS context".to_string())
        }
    }

    pub fn execute_source(&mut self, source: &str, bridge: &Arc<Mutex<JsBridge>>) -> Result<(), String> {
        if let Some(ref ctx) = self.context {
            ctx.with(|ctx: rquickjs::Ctx<'_>| {
                let _ = js_bridge::register_browser_api(&ctx, bridge);
                ctx.eval::<(), _>(source).map_err(|e| format!("JS Error: {:?}", e))
            })
        } else {
            Err("No JS context".to_string())
        }
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
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
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