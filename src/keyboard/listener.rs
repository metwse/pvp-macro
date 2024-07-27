use std::{
    fs, io, 
    sync::{
        Arc, Mutex,
    }
};

use super::{run, minecraft};

use crate::data_dir;
use rdev::{
    listen,
    Event, EventType
};

/// Listens keyboard and manages macro.
pub struct Listener {
    listening: Mutex<bool>,
    running: Mutex<bool>,
    pub service: Arc<run::MacroService>,
    pub minecraft: Arc<minecraft::Minecraft>,
}

impl Listener {
    pub fn new() -> Arc<Self> {
        let minecraft = minecraft::Minecraft::new();
        let service = run::MacroService::new(Arc::clone(&minecraft));
        Arc::clone(&service).init().unwrap();
        Arc::new(
            Self {
                listening: Mutex::new(false),
                running: Mutex::new(false),
                minecraft,
                service,
            }
        )
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
                            self.service.use_item(slot.clone());
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

    pub fn is_listening(&self) -> bool { *self.listening.lock().unwrap() }

    pub fn is_running(&self) -> bool { *self.running.lock().unwrap() }

    pub fn save_settings(&self) {
        use crate::keyboard::SaveJson;
        let mut files: Vec<_> = ["settings.json", "keybindings.json"]
            .iter()
            .map(|file| fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(data_dir().join(file))
                .unwrap()
            )
            .collect();
        self.service.settings.lock().unwrap().to_json(&mut files[0]).unwrap();
        self.minecraft.keybindings.lock().unwrap().to_json(&mut files[1]).unwrap();
    }
    
    pub fn load_settings(&self) {
        use crate::keyboard::SaveJson;
        let files: Vec<_> = ["settings.json", "keybindings.json"]
            .iter()
            .map(|file| fs::File::open(data_dir().join(file)))
            .collect();

        if let Ok(file) = &files[0] {
            let mut reader = io::BufReader::new(file);
            *self.service.settings.lock().unwrap() = run::Settings::from_json(&mut reader);
        }

        if let Ok(file) = &files[1] {
            let mut reader = io::BufReader::new(file);
            *self.minecraft.keybindings.lock().unwrap() = minecraft::KeyBindings::from_json(&mut reader);
        }
    }
}
