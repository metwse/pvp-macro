use pvp_macro::{ 
    ui::UI,
    keyboard
};
use std::{
    thread,
    time::Duration,
    sync::Arc,
    fs
};

fn main() {
    fs::create_dir(pvp_macro::data_dir()).unwrap_or(());

    let listener = keyboard::Listener::new();
    //listener.load_keybindings(data_dir().join("keybindings.json")).unwrap();
    listener.load_settings();

    let listener2 = Arc::clone(&listener);
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        listener2.listen();
    });

    let ui = UI::new(Arc::clone(&listener));
    ui.init();
    ui.run();
}
