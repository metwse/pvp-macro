use pvp_macro::ui;
use pvp_macro::keyboard;
use std::{
    thread,
    time::Duration,
    sync::Arc
};

fn main() {
    let listener = keyboard::listener::MacroListener::new();

    let listener2 = Arc::clone(&listener);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        listener2.listen();
    });

    ui::run(ui::init(listener))
}
