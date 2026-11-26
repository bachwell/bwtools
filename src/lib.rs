mod bwlog;
mod dirs;

pub use crate::bwlog::LogBuilder;

pub use crate::dirs::ApplicationDirs;

pub use log;

pub const CARGO_PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
pub const CARGO_PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
