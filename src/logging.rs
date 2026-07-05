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
    if let Err(e) = create_dir_all(&p) {
        eprintln!("[logging] failed to create log dir {:?}: {}", p, e);
    }
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
            if let Err(e) = writeln!(f, "═══ Aether Browser Pipeline Log ═══") {
                eprintln!("[logging] write failed: {}", e);
            }
            if let Err(e) = writeln!(f, "Started: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")) {
                eprintln!("[logging] write failed: {}", e);
            }
            if let Err(e) = writeln!(f, "────────────────────────────────────") {
                eprintln!("[logging] write failed: {}", e);
            }
        }
        PipelineLog { file, start: Instant::now(), enabled: false }
    }

    pub(crate) fn write(&mut self, section: &str, msg: &str) {
        if !self.enabled { return; }
        let elapsed = self.start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        if let Some(ref mut f) = self.file {
            if let Err(e) = writeln!(f, "[{:>10.3}ms][{}] {}", ms, section, msg) {
                eprintln!("[logging] write failed: {}", e);
            }
            if let Err(e) = f.flush() {
                eprintln!("[logging] flush failed: {}", e);
            }
        }
    }
}

pub fn set_enabled(enabled: bool) {
    if let Ok(mut log) = LOGGER.lock() {
        log.enabled = enabled;
        if enabled {
            if let Some(f) = log.file.as_mut() {
                if let Err(e) = writeln!(f, "═══ Logging ENABLED ═══") {
                    eprintln!("[logging] write failed: {}", e);
                }
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

#[allow(dead_code)] // ponytail: public API, useful for forcing LazyLock init early
pub fn init() {
    LazyLock::force(&LOGGER);
}
