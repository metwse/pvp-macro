use core::panic;
use std::sync::{Mutex, Arc};

use super::{MacroService, minecraft};

use rdev::{
    listen,
    Event, EventType
};


pub struct MacroListener {
    service: Arc<MacroService>,
    listening: Mutex<bool>,
    running: Mutex<bool>,
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
    /// or [`MacroService::start`] returns `Err`.
    ///
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
    /// or [`MacroService::pause`] returns `Err`.
    ///
    pub fn stop(&self) -> Result<(), String> {
        if !self.is_running() { return Err(String::from("Macro is already running")); }
        *self.running.lock().unwrap() = false;
        Ok(())
    }
}
