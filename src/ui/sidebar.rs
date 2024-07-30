use super::{
    menus,
    UI,
    Theme,
    util::*
};

use fltk::{prelude::*, *};

use std::sync::Arc;

#[derive(PartialEq)]
pub enum Menu {
    Run, Settings, KeyBindings, Info, Metw
}

enum MenuFn {
    Standard (fn(crate::ui::menus::MenuFrame) -> (), ),
    Macro (fn(crate::ui::menus::MenuFrame, Arc<crate::keyboard::Listener>) -> (), ),
    NoArg (fn() -> (), ),
}
static MENU_DATA: [(Menu, &str, MenuFn); 5] = [
    (Menu::Run, "sidebar/run.svg", MenuFn::Macro(menus::run)),
    (Menu::Settings, "sidebar/settings.svg", MenuFn::Macro(menus::settings)),
    (Menu::KeyBindings, "sidebar/keybindings.svg", MenuFn::Macro(menus::keybindings)),
    (Menu::Info, "sidebar/info.svg", MenuFn::Standard(menus::info)),
    (Menu::Metw, "sidebar/metw.svg", MenuFn::NoArg(menus::metw)),
];


impl UI {
    pub fn init_sidebar(self: &Arc<Self>) {
        let mut sidebar = self.sidebar.lock().unwrap();
        sidebar.begin();
        for (menu_kind, asset, _) in MENU_DATA.iter() {
            let mut button = button::Button::default();
            sidebar.fixed(&button, 44);
            btn_cursor(&mut button);
            button.set_frame(enums::FrameType::RFlatBox);
            button.set_color(Theme::NAVBAR_BG);
            button.set_selection_color(Theme::NAVBAR_BG.lighter().darker());

            let mut image = get_svg(asset);
            image.scale(36, 36, true, true);
            button.set_image(Some(image));

            let ui = Arc::clone(&self);
            button.set_callback(move |_| {
                ui.select_menu(menu_kind)
            });
        }
        sidebar.end();
    }

    pub fn select_menu(&self, menu: &Menu) {
        for (menu2, _, function) in MENU_DATA.iter() {
            if menu == menu2 {
                let root = self.root.lock().unwrap();
                self.listener.stop().unwrap_or(());

                if let MenuFn::NoArg(function) = function {
                    return function()
                }

                if let Some(wid) = self.current_menu.lock().unwrap().take() {
                    app::delete_widget(wid);
                }

                root.begin();
                let mut menu = group::Flex::default();
                menu.set_margin(8);
                root.end();

                match function {
                    MenuFn::Standard(function) => function(&mut menu),
                    MenuFn::Macro(function) => function(&mut menu, Arc::clone(&self.listener)),
                    _ => unreachable!()
                }

                *self.current_menu.lock().unwrap() = Some(menu);
                app::redraw();
                break
            }
        }
    }
}
