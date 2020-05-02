//! A logger that prints all messages in browser's console.
//!
//! By default, `web_logger` will use the `std_web` feature, which depends on `stdweb`. If you want to use `web-sys`,
//! add this to your Cargo.toml under `[dependencies]`:
//! ```toml
//! web_logger = { version="0.2", default-features=false, features="web_sys" }
//! ```

use log::{Level, Log, Metadata, Record, SetLoggerError};

cfg_if::cfg_if! {
    if #[cfg(feature = "std_web")] {
        mod std_web;
        use std_web::console;
    } else if #[cfg(feature = "web_sys")] {
        mod web_sys;
        use crate::web_sys::console;
    }
}

pub struct Config {
    pub level: Level,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            level: Level::Trace,
        }
    }
}

static LOGGER: WebLogger = WebLogger;

struct WebLogger;

impl Log for WebLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        // TODO Check the args of a location
        true
    }

    fn log(&self, record: &Record) {
        let metadata = record.metadata();
        if self.enabled(metadata) {
            let msg = format!(
                "{}:{} -- {}",
                record.level(),
                record.target(),
                record.args()
            );
            match metadata.level() {
                Level::Trace => console::trace(&msg),
                Level::Debug => console::debug(&msg),
                Level::Info => console::info(&msg),
                Level::Warn => console::warn(&msg),
                Level::Error => console::error(&msg),
            }
        }
    }

    fn flush(&self) {}
}

pub fn try_init(config: Config) -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER)?;
    let level = config.level;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

pub fn init() {
    try_init(Config::default())
        .expect("web_logger::init should not be called after logger initialized");
}

pub fn custom_init(config: Config) {
    try_init(config)
        .expect("web_logger::custom_init should not be called after logger initialized");
}
