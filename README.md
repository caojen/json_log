# Logger with Json Format

`json_log` helpers you to print logs in `json` format.

## Usage

First, add these in your `Cargo.toml`:

```toml
[dependencies]
log = { version = "0.4" }
json_log = { version = "0.1" }

serde = { version = "1", features = ["derive"] } # if you want to log your struct...
```

You need to choose the logging level after the program starts. For example:
```rust
use log::LevelFilter;

fn main() {
    json_log::init_with_level(LevelFilter::Info).unwrap(); // use the `INFO` level.
}
```

Or, you can set environment variable `RUST_LOG`:
```rust
fn main() {
    // Note: set environment variable `RUST_LOG` to one of these (case insensitive):
    let accepted_RUST_LOG = ["Trace", "Debug", "Info", "Warn", "Error"];
    
    json_log::init_env().unwrap();
}
```

### Simple Text Logging

`json_log` is an adapter of the `log` trait. You can use all the `log` features in your code.

For example:
```rust
use log::LevelFilter;
use log::{debug, info, warn, error};

fn main() {
    json_log::init_with_level(LevelFilter::Debug);
    
    let a = 1 + 2 + 3;
    
    debug!("debug");
    info!("a = {a}");
    if a == 6 {
        warn!("a is {}", a);
    } else {
        error!("unknown error: a should be {}, but got {a}", 6);
    }
}
```

And here is the output:
```text
{"level":"Debug","ts":1695181245461,"msg":"debug"}
{"level":"Info","ts":1695181245461,"msg":"a = 6"}
{"level":"Warn","ts":1695181245461,"msg":"a is 6"}
```

### Log your struct
If you have already defined your own logging struct, you can log it by make it `Serialize`. For example:
```rust
use serde::Serialize;

#[derive(Serialize)]
struct MyLoggingStruct {
    pub status: u16,
    pub username: String,
    pub method: String,
    pub headers: Map<String, String>,
}

// Or, ...
#[derive(Serialize)]
struct MyLoggingStructV2<'a> {
    pub status: u16,
    pub username: &'a str,
    pub method: &'a str,
    pub headers: Map<&'a str, &'a str>,
}
```

Then, do the log:

```rust
use std::collections::HashMap;
use log::LevelFilter;

// A static reference, you can define it everywhere.
static MY_LOGGER: &json_log::JsonLogger = json_log::get_default_logger();

fn main() {
    json_log::init_with_level(LevelFilter::Debug); // set to debug level

    let data = MyLoggingStruct {
        status: 404,
        username: "My name is Jack".to_string(),
        method: "ListMyMoney".to_string(),
        headers: HashMap::new(),
    };

    MY_LOGGER.info(&data);
}
```

For more examples, see `./examples`.
