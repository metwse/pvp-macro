mod menus;
mod theme;
mod util;
mod key_picker;

use fltk::{prelude::*, *};

use theme::Theme;
use util::*;

use crate::keyboard::MacroListener;
use std::sync::Arc;

pub fn init(listener: Arc<MacroListener>) -> app::App {
    let a = app::App::default();
    
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

    let mut win = window::Window::default().with_label("pvp macro").with_size(248, 324).with_id("window");
    win.set_border(false);

    let mut root = group::Flex::default().size_of(&win);
    root.set_type(group::FlexType::Column);
    root.set_spacing(0);

    let mut navbar = group::Flex::default();
    root.fixed(&navbar, 24);
    navbar.set_color(Theme::NAVBAR_BG);
    navbar.set_frame(enums::FrameType::FlatBox);
    navbar.set_spacing(0);

    let mut title = frame::Frame::default().with_label("PvP Macro by metwse");
    title.set_label_color(Theme::COLOR);
    navbar.fixed(&title, title.measure_label().0 + 8);

    let _ = frame::Frame::default(); // filler

    // navbar buttons
    for (label, selection_color, index) in [
        ("‒", Theme::BG_1, 1),
        ("×", Theme::NAVBAR_QUIT_BG, 2),
    ] {
        let mut btn = button::Button::default().with_label(label);
        navbar.fixed(&btn, 24);
        btn.set_callback(move |_| {
            match index {
                1 => { 
                    let mut win = app::widget_from_id::<window::Window>("window").unwrap();
                    win.iconize();
                },
                2 => app::quit(),
                _ => unreachable!(),
            };
        });
        btn.set_frame(enums::FrameType::FlatBox);
        btn.set_color(Theme::NAVBAR_BG);
        btn.set_selection_color(selection_color);
    }

    navbar.end();

    // {{{ drag and drop
    navbar.handle({
        let mut x = 0;
        let mut y = 0;
        let mut win2 = win.clone();
        move |_, event| {
            match event {
                enums::Event::Push => {
                    (x, y) = app::event_coords();
                    true
                },
                enums::Event::Drag => {
                    win2.set_pos(app::event_x() + win2.x_root() - x, app::event_y() + win2.y_root() - y);
                    true
                },
                _ => false,
            }
        }
    });
    // }}}

    let mut root2 = group::Flex::default();
    root2.set_type(group::FlexType::Row);
    root2.set_spacing(0);

    let mut sidebar = group::Flex::default();
    root2.fixed(&sidebar, 48);
    sidebar.set_spacing(8);
    sidebar.set_type(group::FlexType::Column);
    sidebar.set_color(Theme::NAVBAR_BG);
    sidebar.set_frame(enums::FrameType::FlatBox);
    sidebar.set_margin(2);

    root2.begin();
    let mut menu = group::Flex::default().with_id("menu");
    menu.set_margin(8);
    menus::info(&mut menu);
    root2.end();


    // {{{ sidebar buttons
    enum MenuFn {
        Standard (fn(crate::ui::menus::MenuFrame) -> (), ),
        Macro (fn(crate::ui::menus::MenuFrame, Arc<crate::keyboard::MacroListener>) -> (), ),
        NoArg (fn() -> (), ),
    }

    sidebar.begin();
    for (asset, function) in 
        [
            ("sidebar/run.svg", MenuFn::Macro(menus::run)),
            ("sidebar/settings.svg", MenuFn::Standard(menus::settings)),
            ("sidebar/keybindings.svg", MenuFn::Standard(menus::keybindings)),
            ("sidebar/info.svg", MenuFn::Standard(menus::info)),
            ("sidebar/metw.svg", MenuFn::NoArg(menus::metw)),
        ]
    {
        listener.stop().unwrap_or(());

        let mut button = button::Button::default();
        sidebar.fixed(&button, 44);
        btn_cursor(&mut button);
        button.set_frame(enums::FrameType::RFlatBox);
        button.set_color(Theme::NAVBAR_BG);
        button.set_selection_color(Theme::NAVBAR_BG.lighter().darker());

        let mut image = get_svg(asset);
        image.scale(36, 36, true, true);
        button.set_image(Some(image));

        let root2 = root2.clone();
        let listener2 = Arc::clone(&listener);
        button.set_callback(move |_| {
            if let MenuFn::NoArg(function) = function {
                return function()
            }

            if let Some(wid) = app::widget_from_id::<group::Flex>("menu") {
                app::delete_widget(wid);
            }

            root2.begin();
            let mut menu = group::Flex::default().with_id("menu");
            menu.set_margin(8);
            root2.end();

            match function {
                MenuFn::Standard(function) => function(&mut menu),
                MenuFn::Macro(function) => function(&mut menu, Arc::clone(&listener2)),
                _ => unreachable!()
            }

            app::redraw();
        });
    }
    sidebar.end();
    // }}}

    root.end();
    win.end();
    win.show();

    win.set_callback(|_| {
        if app::event() == enums::Event::Close {
            app::quit()
        }
    });
    
    a
}

pub fn run(a: app::App) {
    a.run().unwrap();
}
