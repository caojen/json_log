use serde::Serialize;

#[derive(Serialize)]
pub struct Msg<'a, T>
    where T: Serialize
{
    level: &'static str,
    ts: u128,
    msg: &'a T,
}

impl<'a, T> Msg<'a, T>
    where T: Serialize
{
    #[inline]
    fn level_to_string(level: log::Level) -> &'static str {
        match level {
            log::Level::Trace => "Trace",
            log::Level::Debug => "Debug",
            log::Level::Info => "Info",
            log::Level::Warn => "Warn",
            log::Level::Error => "Error",
        }
    }

    pub fn new(level: log::Level, msg: &'a T) -> Msg<'a, T> {
        Self {
            level: Self::level_to_string(level),
            ts: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis(),
            msg,
        }
    }
}
