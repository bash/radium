use log::{set_logger, Log, LogLevelFilter, LogMetadata, LogRecord, SetLoggerError};
use std::io::{stderr, Write};

pub struct Logger;

impl Logger {
    pub fn init() -> Result<(), SetLoggerError> {
        set_logger(|max_log_level| {
            max_log_level.set(LogLevelFilter::Debug);
            Box::new(Logger)
        })
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool { true }

    fn log(&self, record: &LogRecord) {
        writeln!(&mut stderr(), "{} | {}", record.level(), record.args());
    }
}