#![cfg(test)]

use std::collections::HashMap;
use log::{debug, error, info, LevelFilter, SetLoggerError, trace, warn};
use serde::Serialize;

use crate::{get_default_logger, init_with_level, JsonLogger};

static MY_LOGGER: &JsonLogger = get_default_logger();

fn init() -> Result<(), SetLoggerError> {
    log::set_logger(MY_LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Error))
}

fn test_all_log<T: Serialize>(msg: T) {
    MY_LOGGER.trace(&msg);
    MY_LOGGER.debug(&msg);
    MY_LOGGER.info(&msg);
    MY_LOGGER.warn(&msg);
    MY_LOGGER.error(&msg);
}

#[test]
fn test_simple() {
    init().unwrap();

    trace!("trace");
    debug!("debug");
    info!("info");
    warn!("warn");
    error!("error");
}

#[derive(Serialize)]
struct SimpleStruct {
    pub a: String,
    pub b: u8,
    pub c: u32,
    pub d: i64,
    pub e: bool,
}

#[test]
fn test_simple_struct() {
    init().unwrap();

    let s = SimpleStruct {
        a: "Hello World".to_string(),
        b: 1,
        c: 10231,
        d: -12302,
        e: true,
    };

    test_all_log(s);
}

#[test]
fn test_hashmap() {
    init().unwrap();

    let map = {
        let mut map = std::collections::HashMap::new();

        map.insert("a", "bcd");
        map.insert("c", "efs");
        map.insert("en", "Hello World");
        map.insert("zh", "你好，世界");
        map.insert("ja", "こんにちは世界");

        map
    };

    test_all_log(&map);
    test_all_log(&map);
}

#[derive(Serialize)]
struct HttpFull {
    version: String,
    method: String,
    path: String,
    queries: HashMap<String, String>,
    headers: HashMap<String, Vec<String>>,

    status: u16,
    time: usize,
}

#[test]
fn test_http_full() {
    init().unwrap();

    let http = HttpFull {
        version: "HTTP/1.1".to_string(),
        method: "GET".to_string(),
        path: "/api/v1/users/10000".to_string(),
        queries: [("info".to_string(), "full".to_string()), ("status".to_string(), "ok".to_string())]
            .iter()
            .cloned()
            .collect(),
        headers: HashMap::new(),

        status: 404,
        time: 8,
    };

    test_all_log(http);
}

#[test]
fn test_example_text() {
    init_with_level(LevelFilter::Debug).unwrap();

    let a = 1 + 2 + 3;

    debug!("debug");
    info!("a = {a}");
    if a == 6 {
        warn!("a is {}", a);
    } else {
        error!("unknown error: a should be {}, but got {a}", 6);
    }
}
