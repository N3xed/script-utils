use std::path::Path;

pub use ::downloader::{download, downloader, progress, verify, Downloader};
pub use anyhow::*;
pub use cmd_lib::*;
pub use log::*;
pub use simplelog;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

pub fn init() {
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

pub trait DownloadExt {
    fn download_to(&self, download_folder: impl AsRef<Path>) -> Result<()>;

    fn download(&self) -> Result<()> {
        self.download_to(std::env::current_dir()?)
    }
}

impl<const N: usize> DownloadExt for [(&str, Option<&dyn AsRef<Path>>); N] {
    fn download_to(&self, download_folder: impl AsRef<Path>) -> Result<(), Error> {
        let mut downloader = Downloader::builder()
            .download_folder(download_folder.as_ref())
            .build()?;

        let downloads = self
            .iter()
            .map(|(url, opt_filename)| {
                let mut d = download::Download::new(url);
                if let Some(filename) = opt_filename {
                    d = d.file_name(filename.as_ref());
                }
                d
            })
            .collect::<Vec<_>>();

        downloader.download(&downloads)?;

        Ok(())
    }
}