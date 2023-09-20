use log::LevelFilter;
use log::{debug, info, warn, error};
use serde::Serialize;

fn main() {
    json_log::init_with_level(LevelFilter::Debug).unwrap();

    simple_text();
    structures();
}

fn simple_text() {
    let a = 1 + 2 + 3;

    debug!("debug");
    info!("a = {a}");
    if a == 7 {
        warn!("a is {}", a);
    } else {
        error!("unknown error: a should be {}, but got {a}", 7);
    }
}

static MY_LOGGER: &json_log::JsonLogger = json_log::get_default_logger();

fn structures() {
    #[derive(Serialize)]
    struct MyLoggingStructV2<'a> {
        pub status: u16,
        pub username: &'a str,
        pub method: &'a str,
        pub headers: std::collections::HashMap<&'a str, &'a str>,
    }

    let data = MyLoggingStructV2 {
        status: 404,
        username: "My name is Jack",
        method: "ListMyMoney",
        headers: std::collections::HashMap::new(),
    };

    MY_LOGGER.info(&data);
}