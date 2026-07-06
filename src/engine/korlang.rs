use std::sync::{Arc, Mutex};

static WINDOW_TITLE: Mutex<Option<String>> = Mutex::new(None);
static PENDING_NAVIGATION: Mutex<Option<String>> = Mutex::new(None);

pub fn take_window_title() -> Option<String> {
    WINDOW_TITLE.lock().ok()?.take()
}

pub fn take_navigation_url() -> Option<String> {
    PENDING_NAVIGATION.lock().ok()?.take()
}

// ponytail: thin wrapper; replace with a shared VM pool when throughput matters
pub fn eval_korlang(source: &str) -> Result<String, String> {
    let bytecode = korlang::compile(source);
    if bytecode.is_empty() {
        return Err("Compilation produced no bytecode".into());
    }
    let mut vm = korlang::VirtualMachine::new();
    register_default_callbacks(&mut vm);
    vm.execute(bytecode);
    if let Some(val) = vm.stack.last() {
        Ok(val.to_string_val())
    } else {
        Ok("none".into())
    }
}

pub fn register_default_callbacks(vm: &mut korlang::VirtualMachine) {
    vm.register_native("String.split", Arc::new(|args: &[korlang::Value]| {
        if let (Some(korlang::Value::String(s)), Some(korlang::Value::String(sep))) = (args.get(0), args.get(1)) {
            let parts: Vec<korlang::Value> = s.split(sep).map(|p| korlang::Value::String(p.to_string())).collect();
            return korlang::Value::List(parts);
        }
        korlang::Value::List(vec![])
    }));
    vm.register_native("String.replace", Arc::new(|args: &[korlang::Value]| {
        if let (Some(korlang::Value::String(s)), Some(korlang::Value::String(from)), Some(korlang::Value::String(to))) = (args.get(0), args.get(1), args.get(2)) {
            return korlang::Value::String(s.replace(from, to));
        }
        korlang::Value::None
    }));
        vm.register_native("String.split", Arc::new(|args: &[korlang::Value]| {
        if let (Some(korlang::Value::String(s)), Some(korlang::Value::String(sep))) = (args.get(0), args.get(1)) {
            let parts: Vec<korlang::Value> = s.split(sep).map(|p| korlang::Value::String(p.to_string())).collect();
            return korlang::Value::List(parts);
        }
        korlang::Value::List(vec![])
    }));
    vm.register_native("String.replace", Arc::new(|args: &[korlang::Value]| {
        if let (Some(korlang::Value::String(s)), Some(korlang::Value::String(from)), Some(korlang::Value::String(to))) = (args.get(0), args.get(1), args.get(2)) {
            return korlang::Value::String(s.replace(from, to));
        }
        korlang::Value::None
    }));
    vm.register_native("print", Arc::new(|args: &[korlang::Value]| {
        let s: Vec<String> = args.iter().map(|v| v.to_string_val()).collect();
        println!("[korlang] {}", s.join(" "));
        korlang::Value::None
    }));
    vm.register_native("chrome.render", Arc::new(|args: &[korlang::Value]| {
        if let Some(korlang::Value::String(source)) = args.first() {
            let bytecode = korlang::compile(source);
            if !bytecode.is_empty() {
                let mut render_vm = korlang::VirtualMachine::new();
                register_default_callbacks(&mut render_vm);
                render_vm.execute(bytecode);
                if let Some(result) = render_vm.stack.pop() {
                    return result;
                }
            }
        }
        korlang::Value::Bool(true)
    }));
    vm.register_native("chrome.setTitle", Arc::new(|args: &[korlang::Value]| {
        if let Some(korlang::Value::String(title)) = args.first() {
            if let Ok(mut t) = WINDOW_TITLE.lock() {
                *t = Some(title.clone());
            }
        }
        korlang::Value::None
    }));
    vm.register_native("chrome.navigate", Arc::new(|args: &[korlang::Value]| {
        if let Some(korlang::Value::String(url)) = args.first() {
            if let Ok(mut nav) = PENDING_NAVIGATION.lock() {
                *nav = Some(url.clone());
            }
        }
        korlang::Value::Bool(true)
    }));
}
