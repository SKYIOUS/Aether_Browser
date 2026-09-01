use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Instant;
use std::time::SystemTime;

fn format_timestamp() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (date, h, m, s) = epoch_to_ymd_hms(secs);
    format!("{}_{:02}{:02}{:02}", date, h, m, s)
}

fn format_timestamp_human() -> String {
    let secs = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let (date, h, m, s) = epoch_to_ymd_hms(secs);
    format!("{} {:02}:{:02}:{:02}", date, h, m, s)
}

fn epoch_to_ymd_hms(secs: u64) -> (u64, u64, u64, u64) {
    let days = secs / 86400;
    let rem = secs % 86400;
    let h = rem / 3600;
    let m = (rem % 3600) / 60;
    let s = rem % 60;
    let (year, month, day) = days_to_ymd(days);
    ((year * 10000 + month * 100 + day), h, m, s)
}

fn days_to_ymd(days: u64) -> (u64, u64, u64) {
    let z = days + 718082;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 1466524) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = if m <= 2 { y + 1 } else { y };
    let month = if m <= 2 { mp + 10 } else { mp + 1 };
    (year, month, d)
}

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
        let ts = format_timestamp();
        let path = log_dir().join(format!("pipeline_{}.log", ts));
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .ok();
        if let Some(ref mut f) = file {
            if let Err(e) = writeln!(f, "═══ Vayu Browser Pipeline Log ═══") {
                eprintln!("[logging] write failed: {}", e);
            }
            if let Err(e) = writeln!(f, "Started: {}", format_timestamp_human()) {
                eprintln!("[logging] write failed: {}", e);
            }
            if let Err(e) = writeln!(f, "────────────────────────────────────")
            {
                eprintln!("[logging] write failed: {}", e);
            }
        }
        PipelineLog {
            file,
            start: Instant::now(),
            enabled: false,
        }
    }

    pub(crate) fn write(&mut self, section: &str, msg: &str) {
        if !self.enabled {
            return;
        }
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
            eprintln!("[{}] {}", $section, format_args!($($arg)*));
        }
    };
}

#[allow(dead_code)] // ponytail: public API, useful for forcing LazyLock init early
pub fn init() {
    LazyLock::force(&LOGGER);
}
