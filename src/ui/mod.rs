mod menus;
mod theme;
mod util;
mod sidebar;

use fltk::{prelude::*, *};

use theme::Theme;

use crate::keyboard::Listener;
use std::sync::{
    Arc,
    Mutex
};

pub struct UI {
    listener: Arc<Listener>,
    app: app::App,
    window: Mutex<window::Window>,
    sidebar: Mutex<group::Flex>,
    root: Mutex<group::Flex>,
    current_menu: Mutex<Option<group::Flex>>,
}

impl UI {
    pub fn new(listener: Arc<Listener>) -> Arc<Self> {
        let window = window::Window::default()
            .with_label("PvP Macro")
            .with_size(248, 300);
        window.end();

        let mut root = group::Flex::default();
        root.set_type(group::FlexType::Row);
        root.set_spacing(0);

        let mut sidebar = group::Flex::default();
        root.fixed(&sidebar, 48);
        sidebar.set_spacing(8);
        sidebar.set_type(group::FlexType::Column);
        sidebar.set_color(Theme::NAVBAR_BG);
        sidebar.set_frame(enums::FrameType::FlatBox);
        sidebar.set_margin(2);
        sidebar.end();
        root.end();

        Arc::new(Self {
            listener,
            app: app::App::default(),
            window: Mutex::new(window),
            sidebar: Mutex::new(sidebar),
            root: Mutex::new(root),
            current_menu: Mutex::new(None),
        })
    }

    pub fn init(self: &Arc<Self>) {
        // default colors
        {
            let (r, g, b) = Theme::COLOR.to_rgb();
            app::set_color(Theme::COLOR, r, g, b);
            
            let (r, g, b) = Theme::BG_1.to_rgb();
            app::set_background_color(r, g, b);
            
            let (r, g, b) = Theme::BG_2.to_rgb();
            app::set_background2_color(r, g, b);
        }
        app::set_visible_focus(false);

        let mut window = self.window.lock().unwrap();
        let root = self.root.lock().unwrap();
        window.begin();

        let mut filler_root = group::Flex::default()
            .size_of(&*window);
        filler_root.set_type(group::FlexType::Row);
        filler_root.set_spacing(0);

        filler_root.insert(&*root, 0);

        filler_root.end();
        window.end();
        window.show();
        drop(root);

        self.init_sidebar();
        self.select_menu(&sidebar::Menu::Info);

        window.set_callback(|_| {
            if app::event() == enums::Event::Close {
                app::quit()
            }
        });
    }

    pub fn run(&self) {
        self.app.run().unwrap();
    }
}
