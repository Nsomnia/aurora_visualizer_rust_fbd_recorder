//! Debug logging utilities
//! Version: 0.1.3

use log::{LevelFilter, Record, SetLoggerError};

pub struct DebugLogger;

impl log::Log for DebugLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

impl DebugLogger {
    pub fn init() -> Result<(), SetLoggerError> {
        log::set_logger(&DebugLogger).map(|()| log::set_max_level(LevelFilter::Debug))
    }
}