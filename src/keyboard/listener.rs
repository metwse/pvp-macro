use std::{
    fs, io, 
    sync::{
        Arc, Mutex,
        Condvar
    }
};

use super::{run, minecraft};

use crate::data_dir;
use rdev::{
    listen,
    Event, EventType,
    Key
};

/// Listens keyboard and manages macro.
pub struct Listener {
    listening: Mutex<bool>,
    running: Mutex<bool>,
    pub service: Arc<run::MacroService>,
    pub minecraft: Arc<minecraft::Minecraft>,
    event_key: Arc<(Mutex<bool>, Mutex<Option<Key>>, Condvar)>
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
                event_key: Arc::new((Mutex::new(false), Mutex::new(None), Condvar::new()))
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
    
    pub fn is_listening_key_event(&self) -> bool {
        *self.event_key.0.lock().unwrap()
    }

    fn callback(&self, event: Event) {
        let keybindings = self.minecraft.keybindings.lock().unwrap();

        match event.event_type {
            EventType::KeyPress(key) => {
                if self.is_listening_key_event(){
                    let (running, lock, cvar) = &*Arc::clone(&self.event_key);
                    *lock.lock().unwrap() = Some(key);
                    *running.lock().unwrap() = false;
                    cvar.notify_one();
                }

                if !self.is_running() { return }
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
                if !self.is_running() { return }
                if key == keybindings.start { self.service.pause().unwrap_or(()) }
            },
            EventType::ButtonPress(_) => {
                if self.is_listening_key_event(){
                    let (running, lock, cvar) = &*Arc::clone(&self.event_key);
                    *lock.lock().unwrap() = None;
                    *running.lock().unwrap() = false;
                    cvar.notify_one();
                }
            },
            _ => ()
        }
    }

    pub fn await_key(&self) -> Option<Key> {
        let (running, lock, cvar) = &*self.event_key;
        let mut running = running.lock().unwrap();
        if *running { return None }
        *running = true;
        drop(running);
        let key = cvar.wait(lock.lock().unwrap()).unwrap();
        *key
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
