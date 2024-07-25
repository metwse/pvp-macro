use rdev::{
    Key, Button,
    EventType,
    simulate
};

use serde::{Serialize, Deserialize};

use std::{
    cell::RefCell, rc::Rc,
    sync::{ mpsc, Arc, Mutex },
    thread, time::Duration,
};

pub struct Minecraft {
    tx: Arc<mpsc::Sender<Message>>,
    busy: Arc<Mutex<bool>>,
    pub keybindings: Mutex<KeyBindings>,
}

enum Message {
    UseItem(Key, bool),
    PunchItem(Key, bool),
}

impl Minecraft {
    pub fn new() -> Arc<Self> {
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(tx);
        let tx2 = Arc::clone(&tx);

        let minecraft = Arc::new(Self {
            keybindings:  Mutex::new(KeyBindings::default()),
            busy: Arc::new(Mutex::new(false)),
            tx
        });

        let busy = Arc::clone(&minecraft.busy);

        thread::spawn(move || {
            let key_press = Rc::new(RefCell::new(None));
            let button_press = Rc::new(RefCell::new(None));
            let skip = Rc::new(RefCell::new(None));

            let kp2 = Rc::clone(&key_press);
            let bp2 = Rc::clone(&button_press);
            let sk2 = Rc::clone(&skip);
            let send = |events: Vec<&EventType>| {
                for event in events {
                    match event {
                        EventType::KeyPress(key) => *kp2.borrow_mut() = Some(*key),
                        EventType::KeyRelease(_) => *kp2.borrow_mut() = None,
                        EventType::ButtonPress(button) => *bp2.borrow_mut() = Some(*button),
                        EventType::ButtonRelease(_) => *bp2.borrow_mut() = None,
                        _ => unreachable!()
                    }
                    simulate(event).unwrap_or(());
                }
                if let Ok(message) = rx.recv_timeout(Duration::from_millis(30)) {
                    *sk2.borrow_mut() = Some(message);
                    return
                }
            };

            let kp2 = Rc::clone(&key_press);
            let bp2 = Rc::clone(&button_press);
            let release_all = || {
                if let Some(key) = (*kp2.borrow_mut()).take() { simulate(&EventType::KeyRelease(key)).unwrap_or(()); }
                if let Some(button) = (*bp2.borrow_mut()).take() { simulate(&EventType::ButtonRelease(button)).unwrap_or(()); }
                thread::sleep(Duration::from_millis(20));
            };

            while let Ok(message) = rx.recv() {
                match message {
                    Message::UseItem(slot, strong) => {
                        let mut busy = busy.lock().unwrap();
                        if !strong && *busy { return }
                        if strong { *busy = true }
                        send(vec![&EventType::KeyPress(slot), &EventType::ButtonPress(Button::Right)]);
                        send(vec![&EventType::KeyRelease(slot), &EventType::ButtonRelease(Button::Right)]);
                        if strong { *busy = false }
                    }
                    Message::PunchItem(slot, strong) => {
                        let mut busy = busy.lock().unwrap();
                        if !strong && *busy { return }
                        if strong { *busy = true }
                        send(vec![&EventType::KeyPress(slot), &EventType::ButtonPress(Button::Left)]);
                        send(vec![&EventType::KeyRelease(slot), &EventType::ButtonRelease(Button::Left)]);
                        if strong { *busy = false }
                    }
                }
                if let Some(message) = (*skip.borrow_mut()).take() {
                    release_all();
                    tx2.send(message).unwrap();
                }
            }
        });
        minecraft
    }

    pub fn use_item(&self, slot: Key) {
        self.tx.send(Message::UseItem(slot, true)).unwrap();
    }
    
    pub fn use_fishing_rod(&self) {
        self.tx.send(Message::UseItem(self.keybindings.lock().unwrap().fishing_rod, false)).unwrap();
    }

    pub fn use_sword(&self) {
        self.tx.send(Message::PunchItem(self.keybindings.lock().unwrap().sword, false)).unwrap();
    }

    pub fn load_keybindings(&self, keybindings: KeyBindings) {
        *self.keybindings.lock().unwrap() = keybindings;
    }
}

#[derive(Serialize, Deserialize)]
pub struct KeyBindings {
    pub start: Key,
    pub sword: Key,
    pub fishing_rod: Key,
    pub custom: Vec<[Key; 2]>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            start: Key::ControlLeft,
            sword: Key::Num1,
            fishing_rod: Key::Num2,
            custom: vec![
                [Key::KeyX, Key::Num3],
                [Key::KeyC, Key::Num4],
                [Key::KeyV, Key::Num5],
                [Key::KeyF, Key::Num6],
            ]
        }
    }
}
