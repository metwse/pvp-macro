mod listener;
pub mod run;
pub mod minecraft;

pub use listener::Listener;
pub use run::MacroService;

use std::io;
use serde::{Deserialize, Serialize};

pub trait SaveJson {
    fn from_json(json: &mut dyn io::Read) -> Self where Self: Sized + Default, for<'de> Self: Deserialize<'de> {
        serde_json::from_reader(json).unwrap_or(Self::default())
    }

    fn to_json(&self, writer: &mut impl io::Write) -> Result<(), serde_json::Error> where Self: Serialize {
        serde_json::to_writer(writer, &self)
    }
}

impl SaveJson for minecraft::KeyBindings { }
impl SaveJson for run::Settings { }
