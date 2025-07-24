use std::{env, panic};
use log::LevelFilter;
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};
use anyhow::{Context, Result};

pub fn init() -> Result<()> {
    let current_bin = env::current_exe().context("Failed to get binary path")?;
    let current_dir = current_bin.parent().context("Failed to get binary directory")?;
    let log_path = current_dir.join("7thDeck.log");

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[{h({l})}] {m}{n}")))
        .build();

    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S)} [{l}] {t} - {m}{n}")))
        .append(false)
        .build(log_path)?;

        let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Error)))
                .build("stdout", Box::new(stdout)),
        )
        .build(
            Root::builder()
                .appender("file")
                .appender("stdout")
                .build(LevelFilter::Info),
        )?;

    log4rs::init_config(config)?;

    panic::set_hook(Box::new(|panic_info| {
        let message = if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
            s.to_string()
        } else {
            "Panic occurred (unknown payload)".to_string()
        };

        let location = panic_info.location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()))
            .unwrap_or_else(|| " (location unknown)".to_string());

        log::error!("Panic at {location}: {message}");
    }));

    Ok(())
}
