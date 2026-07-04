pub mod js_bridge;

use std::sync::{Arc, Mutex};
use rquickjs::Runtime as QuickJSRuntime;
use crate::plog;

pub use js_bridge::JsBridge;

pub struct JSEngine {
    context: Option<rquickjs::Context>,
}

impl JSEngine {
    pub fn new() -> Self {
        let runtime = match QuickJSRuntime::new() {
            Ok(r) => r,
            Err(_) => return Self { context: None },
        };
        let ctx = match rquickjs::Context::full(&runtime) {
            Ok(c) => c,
            Err(_) => return Self { context: None },
        };
        Self { context: Some(ctx) }
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
                if let Err(e) = js_bridge::register_browser_api(&ctx, bridge) {
                    plog!("JS", "register_browser_api failed: {:?}", e);
                }
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
                if let Err(e) = js_bridge::register_browser_api(&ctx, bridge) {
                    plog!("JS", "register_browser_api failed: {:?}", e);
                }
                ctx.eval::<(), _>(source).map_err(|e| format!("JS Error: {:?}", e))
            })
        } else {
            Err("No JS context".to_string())
        }
    }
}