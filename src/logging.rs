#![allow(dead_code)]
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::LazyLock;
use std::time::Instant;

pub(crate) static LOGGER: LazyLock<Mutex<PipelineLog>> =
    LazyLock::new(|| Mutex::new(PipelineLog::new()));

fn log_dir() -> PathBuf {
    let mut p = std::env::current_exe().unwrap_or_default();
    p.pop();
    p.push("logs");
    let _ = create_dir_all(&p);
    p
}

pub(crate) struct PipelineLog {
    file: Option<File>,
    start: Instant,
    enabled: bool,
}

impl PipelineLog {
    fn new() -> Self {
        let ts = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let path = log_dir().join(format!("pipeline_{}.log", ts));
        let mut file = OpenOptions::new().create(true).append(true).open(&path).ok();
        if let Some(ref mut f) = file {
            let _ = writeln!(f, "═══ Aether Browser Pipeline Log ═══");
            let _ = writeln!(f, "Started: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
            let _ = writeln!(f, "────────────────────────────────────");
        }
        PipelineLog { file, start: Instant::now(), enabled: false }
    }

    pub(crate) fn write(&mut self, section: &str, msg: &str) {
        if !self.enabled { return; }
        let elapsed = self.start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        if let Some(ref mut f) = self.file {
            let _ = writeln!(f, "[{:>10.3}ms][{}] {}", ms, section, msg);
            let _ = f.flush();
        }
    }
}

pub fn set_enabled(enabled: bool) {
    if let Ok(mut log) = LOGGER.lock() {
        log.enabled = enabled;
        if enabled {
            if let Some(f) = log.file.as_mut() {
                let _ = writeln!(f, "═══ Logging ENABLED ═══");
            }
        }
    }
}

pub fn is_enabled() -> bool {
    LOGGER.lock().map(|l| l.enabled).unwrap_or(false)
}

#[macro_export]
macro_rules! plog {
    ($section:expr, $($arg:tt)*) => {
        if $crate::logging::is_enabled() {
            if let Ok(mut log) = $crate::logging::LOGGER.lock() {
                log.write($section, &format!($($arg)*));
            }
        }
        eprintln!("[{}] {}", $section, format_args!($($arg)*));
    };
}

pub fn init() {
    LazyLock::force(&LOGGER);
}
