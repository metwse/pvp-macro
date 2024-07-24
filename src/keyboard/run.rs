use std::{
     sync::{Arc, Condvar, Mutex}, thread, time::Duration
};

use rand::{thread_rng, Rng};

use super::minecraft::Minecraft;

#[derive(Clone, Copy)]
enum Message {
    None,
    Skip,
    Abort,
    Start,
}

pub struct MacroService {
    sleep_micros: [u64; 2],
    count: [u64; 2],
    random_ratio: f64,
    running: Mutex<bool>,
    initialized: Mutex<bool>,
    park: (Mutex<Message>, Condvar),
    minecraft: Option<Arc<Minecraft>>,
}

impl Default for MacroService {
    fn default() -> Self {
        Self {
            sleep_micros: [76_923, 200_000],
            count: [13, 5],
            random_ratio: 0.2,
            running: Mutex::new(false),
            initialized: Mutex::new(false),
            park: (Mutex::new(Message::None), Condvar::new()),
            minecraft: None,
        }
    }
}

impl MacroService {
    pub fn new(minecraft: Arc<Minecraft>) -> Self {
        Self {
            minecraft: Some(minecraft),
            ..Self::default()
        }
    }

    fn sleep(&self, micros: u64, run: impl Fn() -> ()) -> Message {
        let (lock, cvar) = &self.park;
        let (message, result) = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_micros(micros)).unwrap();
        if !result.timed_out() { return *message; }
        run();

        Message::None
    }

    fn is_running(&self) -> bool {
        *self.running.lock().unwrap()
    }

    fn is_initialized(&self) -> bool {
        *self.initialized.lock().unwrap()
    }

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
                'inner: loop {
                    let (lock, cvar) = &listener.park;
                    let message = cvar.wait(lock.lock().unwrap()).unwrap();

                    match *message {
                        Message::Start => break 'inner,
                        Message::Abort => break 'outer,
                        _ => unreachable!(),
                    }
                }

                'inner: loop {
                    for i in 0..2 {
                        for _ in 0..listener.count[i] {
                            match listener.sleep(((1.0 + rng.gen_range(-listener.random_ratio..=listener.random_ratio)) * listener.sleep_micros[i] as f64).round() as u64, || {
                                if i == 0 {
                                    listener.minecraft.as_ref().unwrap().use_sword();
                                } else {
                                    listener.minecraft.as_ref().unwrap().use_fishing_rod();
                                }
                            }) {
                                Message::Skip => break 'inner,
                                Message::Abort => break 'outer,
                                Message::None => (),
                                _ => unreachable!(),
                            }
                        }
                    }
                }
            }
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

        *self.running.lock().unwrap() = false;
        *self.initialized.lock().unwrap() = false;
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

        *self.running.lock().unwrap() = false;
        self.notify_thread(Message::Skip);
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

        *self.running.lock().unwrap() = true;
        self.notify_thread(Message::Start);
        Ok(())
    }

}
