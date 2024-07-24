use core::panic;
use std::sync::{Mutex, Arc};
use super::run::Service;

use rdev::{
    listen, Key,
    Event, EventType
};

pub struct MacroListener {
    keybindings: Mutex<KeyBindings>,
    service: Arc<Service>,
    listening: Mutex<bool>,
}

struct KeyBindings {
    start: Key,
}


impl MacroListener {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            keybindings: Mutex::new(KeyBindings { start: Key::ControlLeft }),
            service: Arc::new(Service::new()),
            listening: Mutex::new(false),
        })
    }
    
    pub fn is_listening(&self) -> bool {
        *self.listening.lock().unwrap()
    }

    pub fn listen(self: &Arc<Self>) {
        let listener: Arc<_> = Arc::clone(self);
        *listener.listening.lock().unwrap() = true;
        if let Err(err) = listen(move |event| listener.callback(event)) {
            panic!("Error: {:?}", err);
        }
    }

    fn callback(&self, event: Event) {
        let keybindings = self.keybindings.lock().unwrap();
        match event.event_type {
            EventType::KeyPress(key) => {
                match key {
                    _ if key == keybindings.start => {
                        self.service.clone().start()
                    },
                    _ => (),
                }
            },
            EventType::KeyRelease(key) => {
                match key {
                    _ if key == keybindings.start => {
                        self.service.clone().stop()
                    },
                    _ => (),
                }
            },
            _ => ()
        }
    }
}
