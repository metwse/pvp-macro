use pvp_macro::{ 
    ui,
    keyboard,
    data_dir
};
use std::{
    thread,
    time::Duration,
    sync::Arc,
    fs
};

fn main() {
    fs::create_dir(pvp_macro::data_dir()).unwrap_or(());

    let listener = keyboard::MacroListener::new();
    listener.load_keybindings(data_dir().join("keybindings.json")).unwrap();

    let listener2 = Arc::clone(&listener);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        listener2.listen();
    });

    ui::run(ui::init(listener))
}
