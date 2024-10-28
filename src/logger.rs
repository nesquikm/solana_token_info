use colog::basic_builder;
use log::info;
use std::u8;

pub fn init(int_level: u8) {
    let mut builder = basic_builder();

    let level = match int_level {
        0 => log::LevelFilter::Error,
        1 => log::LevelFilter::Warn,
        2 => log::LevelFilter::Info,
        3 => log::LevelFilter::Debug,
        4 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Trace,
    };

    builder.filter(None, level);
    builder.init();

    info!("Setting log level to: {:?}", level);

    log::set_max_level(level);
}
