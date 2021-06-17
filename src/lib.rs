use std::{path::Path, process::Command};

pub use anyhow::*;
pub use log::*;

#[cfg(feature = "logger")]
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

/// Initialize a terminal logger with color support.
#[cfg(feature = "logger")]
pub fn init_logger() {
    TermLogger::init(
        LevelFilter::Trace,
        ConfigBuilder::default()
            .set_max_level(LevelFilter::Info)
            .set_time_level(LevelFilter::Trace)
            .build(),
        TerminalMode::Stdout,
        ColorChoice::Auto,
    )
    .unwrap();
}

/// Download `url` to `file_path` using curl.
/// 
/// Note: Curl must be installed an in the path.
pub fn download_file(url: impl Into<String>, file_path: impl AsRef<Path>) -> Result<()> {
    let url = format!("\"{}\"", url.into());
    let file_path = format!("\"{}\"", file_path.as_ref().display());
    let args = ["-o", &file_path, &url];
    Command::new("curl").args(&args).spawn()?.wait()?;
    Ok(())
}