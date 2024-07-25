use core::panic;
use std::{
    fs, io, path::PathBuf, sync::{Arc, Mutex}
};

use super::{MacroService, minecraft};
use crate::keyboard::Config;

use rdev::{
    listen,
    Event, EventType
};

pub struct MacroListener {
    listening: Mutex<bool>,
    running: Mutex<bool>,
    service: Arc<MacroService>,
    minecraft: Arc<minecraft::Minecraft>,
}


impl MacroListener {
    pub fn new() -> Arc<Self> {
        let minecraft = minecraft::Minecraft::new();
        let service = Arc::new(MacroService::new(Arc::clone(&minecraft)));
        Arc::clone(&service).init().unwrap();
        Arc::new(
            Self {
                service,
                listening: Mutex::new(false),
                running: Mutex::new(false),
                minecraft,
            }
        )
    }

    pub fn load_keybindings(&self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::File::open(path)?;
        let mut reader = io::BufReader::new(file);
        self.minecraft.load_keybindings(minecraft::KeyBindings::from_json(&mut reader));
        Ok(())
    }
    
    pub fn is_listening(&self) -> bool {
        *self.listening.lock().unwrap()
    }

    pub fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    /// Runs keyboard listener.
    pub fn listen(self: &Arc<Self>) {
        let listener: Arc<_> = Arc::clone(self);
        *listener.listening.lock().unwrap() = true;
        if let Err(err) = listen(move |event| listener.callback(event)) {
            panic!("Error: {:?}", err);
        }
    }

    fn callback(&self, event: Event) {
        if !self.is_running() { return }
        let keybindings = self.minecraft.keybindings.lock().unwrap();
        match event.event_type {
            EventType::KeyPress(key) => {
                if key == keybindings.start { self.service.start().unwrap_or(()) }
                else {
                    for [hotkey, slot] in keybindings.custom.iter() {
                        if key == *hotkey {
                            self.minecraft.use_item(slot.clone());
                            break;
                        }
                    }
                }
            },
            EventType::KeyRelease(key) => {
                if key == keybindings.start { self.service.pause().unwrap_or(()) }
            },
            _ => ()
        }
    }

    /// Starts macro.
    ///
    /// # Errors 
    ///
    /// Returns `Err` if called while macro already running
    pub fn start(&self) -> Result<(), String> {
        if self.is_running() { return Err(String::from("Macro is already running")); }
        *self.running.lock().unwrap() = true;
        Ok(())
    }

    /// Stops macro.
    ///
    /// # Errors 
    ///
    /// Returns `Err` if called while macro is not running
    pub fn stop(&self) -> Result<(), String> {
        if !self.is_running() { return Err(String::from("Macro is already not running")); }
        *self.running.lock().unwrap() = false;
        Ok(())
    }
}
