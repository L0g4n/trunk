//! Common functionality and types.

use std::path::PathBuf;

use anyhow::{anyhow, Result};
use async_std::path::PathBuf as AsyncPathBuf;
use async_std::task::spawn_blocking;

use console::Emoji;
use indicatif::{ProgressBar, ProgressStyle};

pub static BUILDING: Emoji<'_, '_> = Emoji("📦", "");
pub static SUCCESS: Emoji<'_, '_> = Emoji("✅", "");
pub static ERROR: Emoji<'_, '_> = Emoji("❌", "");
pub static SERVER: Emoji<'_, '_> = Emoji("📡", "");

/// Get the CWD, with more descriptive error handling.
pub async fn get_cwd() -> Result<std::path::PathBuf> {
    std::env::current_dir().map_err(|_| anyhow!("failed to determine current working directory"))
}

/// Ensure the given value for `--public-url` is formatted correctly.
pub fn parse_public_url(val: &str) -> String {
    let prefix = if !val.starts_with('/') { "/" } else { "" };
    let suffix = if !val.ends_with('/') { "/" } else { "" };
    format!("{}{}{}", prefix, val, suffix)
}

/// A utility function to recursively copy a directory.
pub async fn copy_dir_recursive(from_dir: PathBuf, to_dir: PathBuf) -> Result<()> {
    if !AsyncPathBuf::from(&from_dir).exists().await {
        return Ok(());
    }
    spawn_blocking(move || {
        let opts = fs_extra::dir::CopyOptions {
            overwrite: true,
            content_only: true,
            ..Default::default()
        };
        Ok(fs_extra::dir::copy(from_dir, to_dir, &opts)?)
    })
    .await
    .map(|_| ())
}

/// Build system spinner.
pub fn spinner() -> ProgressBar {
    let style = ProgressStyle::default_spinner().template("{spinner} {prefix} trunk | {wide_msg}");
    ProgressBar::new_spinner().with_style(style)
}
