mod assets;
pub use assets::ASSETS_DIR;

pub mod ui;
pub mod keyboard;

pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub static NAME: &str = env!("CARGO_PKG_NAME");
