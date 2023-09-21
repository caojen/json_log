mod msg;
mod test_all;

use log::{Metadata, Record};
use serde::Serialize;

#[non_exhaustive]
pub struct JsonLogger;

const LOGGER: JsonLogger = JsonLogger;

impl log::Log for JsonLogger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return
        }

        let line = format!("{}", record.args());

        LOGGER.do_log(record.level(), line);
    }

    fn flush(&self) {}
}

impl JsonLogger {
    #[allow(clippy::only_used_in_recursion)]
    fn do_log<T: Serialize>(&self, level: log::Level, custom: T) {
        let msg = msg::Msg::new(level, &custom);

        match serde_json::to_string(&msg) {
            Ok(line) => {
                if level <= log::Level::Warn {
                    println!("{}", line);
                } else {
                    eprintln!("{}", line);
                }
            },
            Err(e) => {
                let marshal_error = format!("Marshal Error: {}", e);
                self.do_log(level, marshal_error)
            }
        }
    }
}

impl JsonLogger {
    #[inline]
    fn should_log(&self, level: log::LevelFilter) -> bool {
        if log::max_level() == log::LevelFilter::Off {
            return false;
        }

        return log::max_level() >= level;
    }

    #[inline]
    pub fn trace<T: Serialize>(&self, msg: T) {
        if !self.should_log(log::LevelFilter::Trace) {
            return
        }

        self.do_log(log::Level::Trace, msg)
    }

    #[inline]
    pub fn debug<T: Serialize>(&self, msg: T) {
        if !self.should_log(log::LevelFilter::Debug) {
            return
        }

        self.do_log(log::Level::Debug, msg)
    }

    #[inline]
    pub fn info<T: Serialize>(&self, msg: T) {
        if !self.should_log(log::LevelFilter::Info) {
            return
        }

        self.do_log(log::Level::Info, msg)
    }

    #[inline]
    pub fn warn<T: Serialize>(&self, msg: T) {
        if !self.should_log(log::LevelFilter::Warn) {
            return
        }

        self.do_log(log::Level::Warn, msg)
    }

    #[inline]
    pub fn error<T: Serialize>(&self, msg: T) {
        if !self.should_log(log::LevelFilter::Error) {
            return
        }

        self.do_log(log::Level::Error, msg)
    }
}

pub const fn get_default_logger() -> &'static JsonLogger {
    &LOGGER
}

pub fn init_with_level(level: log::LevelFilter) -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(level))
}

pub fn init_from_env() -> Result<(), log::SetLoggerError> {
    let level = match std::env::var("RUST_LOG") {
        Err(_) => log::LevelFilter::Off,
        Ok(s) => match s.to_lowercase().as_str() {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => log::LevelFilter::Off,
        }
    };

    init_with_level(level)
}
