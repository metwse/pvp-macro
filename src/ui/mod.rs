mod menus;
mod theme;
mod util;

use fltk::{prelude::*, *};

use theme::Theme;
use util::*;


pub fn init() -> app::App {
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

    let mut win = window::Window::default().with_label("pvp macro").with_size(248, 274).with_id("window");
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
    navbar.fixed(&title, 160);

    let _ = frame::Frame::default(); // filler

    // navbar buttons
    let mut minimize_button = button::Button::default().with_label("‒");
    navbar.fixed(&minimize_button, 24);
    minimize_button.set_frame(enums::FrameType::FlatBox);
    minimize_button.set_color(Theme::NAVBAR_BG);
    minimize_button.set_selection_color(Theme::BG_1);
    let mut close_button = button::Button::default().with_label("×");
    navbar.fixed(&close_button, 24);
    close_button.set_frame(enums::FrameType::FlatBox);
    close_button.set_color(Theme::NAVBAR_BG);
    close_button.set_selection_color(Theme::NAVBAR_QUIT_BG);

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


    // {{{ sidebar buttons
    type MenuFn = fn(crate::ui::menus::MenuFrame) -> ();
    for (asset, function) in 
        [
            ("sidebar/settings.svg", menus::settings as MenuFn),
            ("sidebar/run.svg", menus::run as MenuFn), 
            ("sidebar/info.svg", menus::info as MenuFn), 
            ("sidebar/metw.svg", menus::info as MenuFn),
        ]
    {

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
        button.set_callback(move |_| {
            if asset == "sidebar/metw.svg" {
                menus::metw();
                return;
            }

            if let Some(wid) = app::widget_from_id::<group::Flex>("menu") {
                app::delete_widget(wid);
            }

            root2.begin();
            let mut menu = group::Flex::default().with_id("menu").with_size(100, 100);
            menu.set_margin(8);
            root2.end();

            function(&mut menu);
            app::redraw();
        });
    }
    sidebar.end();
    // }}}

    root2.end();
    root.end();
    win.end();
    win.show();

    a
}

pub fn run(a: app::App) {
    a.run().unwrap();
}
