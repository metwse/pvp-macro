use std::{
     f64, sync::{Arc, Condvar, Mutex}, thread, time::Duration
};

use rand::{rngs::ThreadRng, thread_rng, Rng};

#[derive(Clone, Copy)]
enum Message {
    None,
    Skip,
    Abort,
}

pub struct Service {
    sleep_micros: [u64; 2],
    count: [u64; 2],
    random_ratio: f64,
    is_running: Mutex<bool>,
    park: (Mutex<Message>, Condvar),
}

impl Default for Service {
    fn default() -> Self {
        Self {
            sleep_micros: [76_923, 200_000],
            count: [13, 5],
            random_ratio: 0.2,
            is_running: Mutex::new(false),
            park: (Mutex::new(Message::None), Condvar::new()),
        }
    }
}

impl Service {
    pub fn new() -> Self {
        Self::default()
    }

    fn sleep(&self, micros: u64, rng: &mut ThreadRng, run: impl Fn() -> ()) -> Message {
        let (lock, cvar) = &self.park;
        let micros = ((1.0 + rng.gen_range(-self.random_ratio..=self.random_ratio)) * micros as f64).round() as u64;

        let (mut message, result) = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_micros(micros)).unwrap();
        if !result.timed_out() { 
            let msg = *message;
            *message = Message::None;
            return msg; 
        }
        run();
        Message::None
    }

    fn is_running(&self) -> bool {
        *self.is_running.lock().unwrap()
    }

    pub fn start(self: Arc<Self>) {
        let mut is_running = self.is_running.lock().unwrap();
        if *is_running { return }
        *is_running = true;
        let self2 = self.clone();
        thread::spawn(move || {
            let mut rng = thread_rng();

            'outer: while self2.is_running() {
                for i in 0..2 {
                    for _ in 0..self2.count[i] {
                        match self2.sleep(self2.sleep_micros[i], &mut rng, || {
                            println!("{i}")
                        }) {
                            Message::None => (),
                            Message::Skip => {
                                break 'outer;
                            },
                            Message::Abort => {
                                break 'outer;
                            },
                        }
                    }
                }
            }
        });
    }

    pub fn stop(&self) {
        *self.is_running.lock().unwrap() = false;
        let (lock, cvar) = &self.park;
        *lock.lock().unwrap() = Message::Skip;
        cvar.notify_one();
    }
}
