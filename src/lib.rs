mod msg;
mod test_all;

use log::{Metadata, Record};
use serde::Serialize;

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
        let msg = msg::Msg::new(record.level(), &line);

        LOGGER.do_log(record.level(), msg);
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

    pub fn trace<T: Serialize>(&self, msg: T) {
        self.do_log(log::Level::Trace, msg)
    }

    pub fn debug<T: Serialize>(&self, msg: T) {
        self.do_log(log::Level::Debug, msg)
    }

    pub fn info<T: Serialize>(&self, msg: T) {
        self.do_log(log::Level::Info, msg)
    }

    pub fn warn<T: Serialize>(&self, msg: T) {
        self.do_log(log::Level::Warn, msg)
    }

    pub fn error<T: Serialize>(&self, msg: T) {
        self.do_log(log::Level::Error, msg)
    }
}

pub const fn get_default_logger() -> &'static JsonLogger {
    &LOGGER
}
