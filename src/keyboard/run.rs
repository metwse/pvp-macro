use std::{
     sync::{Arc, Condvar, Mutex},
     thread, time::Duration
};

use rand::{thread_rng, Rng};

use serde::{Serialize, Deserialize};

use super::minecraft::Minecraft;



#[derive(Clone, Copy)]
enum Message {
    None, Skip, Abort, Start, Stop,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub sleep_micros: [u64; 2],
    pub count: [u64; 2],
    pub random_ratio: f64,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            sleep_micros: [66_666, 50_000],
            count: [7, 5],
            random_ratio: 0.2,
        }
    }
}



pub struct MacroService {
    pub settings: Arc<Mutex<Settings>>,
    running: Mutex<bool>,
    initialized: Mutex<bool>,
    park: (Mutex<Message>, Condvar),
    minecraft: Option<Arc<Minecraft>>,
}

impl Default for MacroService {
    fn default() -> Self {
        Self {
            settings: Arc::new(Mutex::new(Settings::default())),
            running: Mutex::new(false),
            initialized: Mutex::new(false),
            park: (Mutex::new(Message::None), Condvar::new()),
            minecraft: None,
        }
    }
}

impl MacroService {
    pub fn new(minecraft: Arc<Minecraft>) -> Arc<Self> {
        Arc::new(Self {
            minecraft: Some(minecraft),
            ..Self::default()
        })
    }

    fn sleep(&self, micros: u64, run: impl Fn() -> ()) -> Message {
        let (lock, cvar) = &self.park;
        let (message, result) = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_micros(micros)).unwrap();
        if !result.timed_out() { return *message; }
        run();

        Message::None
    }

    fn is_running(&self) -> bool { *self.running.lock().unwrap() }

    fn is_initialized(&self) -> bool { *self.initialized.lock().unwrap() }

    /// Initializes macro thread.
    ///
    /// # Errors
    ///
    /// Returns `Err` if called while macro thread is already initialized.
    pub fn init(self: Arc<Self>) -> Result<(), String> {
        if self.is_initialized() {
            return Err(String::from("Thread is already initialized"))
        }

        let mut initialized = self.initialized.lock().unwrap();
        *initialized = true;

        let listener = Arc::clone(&self);
        thread::spawn(move || {
            let mut rng = thread_rng();
            
            'outer: loop {
                let (lock, cvar) = &listener.park;
                let message = *cvar.wait(lock.lock().unwrap()).unwrap();
                match message {
                    Message::Start => {
                        *listener.running.lock().unwrap() = true;
                        'inner: loop {
                            for i in 0..2 {
                                let settings = listener.settings.lock().unwrap();
                                for _ in 0..=(settings.count[i] + 1){
                                    match listener.sleep(((1.0 + rng.gen_range(-settings.random_ratio..=settings.random_ratio)) * settings.sleep_micros[i] as f64).round() as u64, || {
                                        if i == 0 {
                                            listener.minecraft.as_ref().unwrap().use_sword();
                                        } else {
                                            listener.minecraft.as_ref().unwrap().use_fishing_rod();
                                        }
                                    }) {
                                        Message::Stop => {
                                            *listener.running.lock().unwrap() = false;
                                            break 'inner
                                        },
                                        Message::Skip => (),
                                        Message::Abort => break 'outer,
                                        Message::None => (),
                                        _ => unreachable!(),
                                    }
                                }
                            }
                        }
                    },
                    Message::Abort => { break 'outer },
                    _ => unreachable!(),
                }
            }
            *listener.running.lock().unwrap() = false;
            *listener.initialized.lock().unwrap() = false;
        });
        
        Ok(())
    }

    fn notify_thread(&self, msg: Message) {
        let (lock, cvar) = &self.park;
        *lock.lock().unwrap() = msg;
        cvar.notify_one();
    }

    /// Aborts macro thread.
    ///
    /// # Errors
    ///
    /// Returns `Err` if called while macro thread is not initialized.
    pub fn abort(&self) -> Result<(), String> {
        if !self.is_initialized() { return Err(String::from("Macro is not initialized")); }

        self.notify_thread(Message::Abort);
        Ok(())
    }

    /// Pauses macro thread.
    ///
    /// # Errors
    ///
    /// Returns `Err` if called while macro thread is not initialized or not running.
    pub fn pause(&self) -> Result<(), String> {
        if !self.is_initialized() { return Err(String::from("Macro is not initialized")); }
        if !self.is_running() { return Err(String::from("Macro is not running")); }

        self.notify_thread(Message::Stop);
        Ok(())
    }

    /// Starts macro thread.
    ///
    /// # Errors
    ///
    /// Returns `Err` if called while macro thread is not initialized or already running.
    pub fn start(&self) -> Result<(), String> {
        if !self.is_initialized() { return Err(String::from("Macro is not initialized")); }
        if self.is_running() { return Err(String::from("Macro is already running")); }

        self.notify_thread(Message::Start);
        Ok(())
    }

    pub fn use_item(&self, slot: rdev::Key) {
        if self.is_running() {
            self.notify_thread(Message::Skip);
        }
        self.minecraft.as_ref().unwrap().use_item(slot);
    }

    pub fn load_settings(&self, settings: Settings) {
        *self.settings.lock().unwrap() = settings;
    }

    pub fn reset_settings(self: &Arc<Self>) {
        *self.settings.lock().unwrap() = Settings::default();
    }
}
