mod assets;
pub use assets::ASSETS_DIR;

pub mod ui;
pub mod keyboard;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static NAME: &str = env!("CARGO_PKG_NAME");

use dirs;
use std::path;
pub fn data_dir() -> path::PathBuf { 
    dirs::config_dir().unwrap().join("pvp-macro")
}
