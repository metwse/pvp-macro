use include_dir::{include_dir, Dir};

pub const ASSETS_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/assets/");
