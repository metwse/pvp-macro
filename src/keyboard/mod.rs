mod listener;
mod run;
pub mod minecraft;

pub use listener::MacroListener;
pub use run::MacroService;

use std::io;
use serde::{Deserialize, Serialize};

trait Config {
    fn from_json(json: &mut dyn io::Read) -> Self where Self: Sized + Default, for<'de> Self: Deserialize<'de> {
        serde_json::from_reader(json).unwrap_or(Self::default())
    }

    fn to_json(&self) -> String where Self: Serialize {
        serde_json::to_string(&self).unwrap()
    }
}

impl Config for minecraft::KeyBindings { }
