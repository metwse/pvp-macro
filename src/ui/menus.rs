use fltk::{prelude::*, *};

use crate::keyboard::{Listener, run};

use std::sync::{ 
    Arc, Mutex
};

use super::{ 
    theme::{self, format_button}, Theme,
    util::* 
};
use webbrowser;

pub type MenuFrame<'a> = &'a mut group::Flex;


pub fn settings(frame: MenuFrame, listener: Arc<Listener>) {
    enum In {
        F (f64, Box<dyn FnMut(&mut run::Settings, f64) -> ()>),
        I (u64, Box<dyn FnMut(&mut run::Settings, u64) -> ()>),
    }

    fn input_num_field(frame: MenuFrame, 
        text: String,
        input: &mut (impl InputExt + WidgetBase),
        default: String,
    ) {
        frame.begin();

        let mut text_label = frame::Frame::default();
        frame.fixed(&text_label, 16);
        text_label.draw(move |b| {
            draw::set_draw_color(Theme::COLOR);
            draw::draw_text(&text, b.x(), b.y() + b.h())
        });
        frame.fixed(input, 24);
        theme::format_input(input);

        frame.add(input);
        frame.end();

        input.set_value(&default[..]);
        frame.end();
    }


    let listener2 = Arc::clone(&listener);
    let load_settings = move |frame: &Arc<Mutex<group::Flex>>| {
        let listener = &listener2;
        let settings = listener.service.settings.lock().unwrap();
        let mut frame = frame.lock().unwrap();
        for i in (0..frame.children()).rev() {
            app::delete_widget(frame.child(i).unwrap());
        }

        let settings_data = [
            (
                "Kılıç CPS", 
                In::F((1.0e8 / (settings.sleep_micros[0] as f64)).round() / 100.0, Box::new(|s, v| {
                    s.sleep_micros[0] = (1.0e6 / v) as u64
                }))
            ),
            (
                "Kılıç vurma sayısı",
                In::I(settings.count[0], Box::new(|s, v| {
                    s.count[0] = v;
                }))
            ),
            (
                "Olta atma sayısı",
                In::I(settings.count[1], Box::new(|s, v| { 
                    s.count[1] = v;
                }))
            ),
            (
                "Olta başına atma süresi",
                In::F(((settings.sleep_micros[1] as f64) / 1.0e4).round() / 100.0, Box::new(|s, v| {
                    s.sleep_micros[1] = (v * 1.0e6) as u64
                }))
            ),
            (
                "Rastgelelik yüzdesi",
                In::F((settings.random_ratio as f64 * 10000.0).round() / 100.0, Box::new(|s, v| {
                    s.random_ratio = v / 100.0
                }))
            ),
            ];

        for (text, input_type) in settings_data.into_iter() {
            let settings = Arc::clone(&listener.service.settings);
            let listener = Arc::clone(&listener);
            match input_type {
                In::F(default, mut cb) => {
                    let mut input = input::FloatInput::default();
                    input_num_field(&mut frame, String::from(text), &mut input, default.to_string());
                    input.handle(move |input, event| {
                        if !matches!(event, enums::Event::KeyDown) { return false }
                        if let Ok(value) = input.value().parse::<f64>() {
                            let mut settings = settings.lock().unwrap();
                            cb(&mut settings, value);
                            input.set_text_color(Theme::COLOR);
                        } else {
                            input.set_text_color(Theme::WARN);
                        }
                        listener.save_settings();
                        true
                    });
                },
                In::I(default, mut cb) => {
                    let mut input = input::IntInput::default();
                    input_num_field(&mut frame, String::from(text), &mut input, default.to_string());
                    input.handle(move |input, event| {
                        if !matches!(event, enums::Event::KeyDown) { return false }
                        if let Ok(value) = input.value().parse::<u64>() {
                            let mut settings = settings.lock().unwrap();
                            cb(&mut settings, value);
                            input.set_text_color(Theme::COLOR);
                        } else {
                            input.set_text_color(Theme::WARN);
                        }
                        listener.save_settings();
                        true
                    });
                },
            }
        }
        app::redraw();
    };
    frame.begin();
    frame.set_type(group::FlexType::Column);

    let frame_mutex = Arc::new(Mutex::new(group::Flex::default()));
    frame_mutex.lock().unwrap().set_type(group::FlexType::Column);
    load_settings(&frame_mutex);

    let _ = frame::Frame::default();
    let mut f = group::Flex::default();
    let _ = frame::Frame::default();
    let mut reset = button::Button::default().with_label("Sıfırla");
    let frame2 = Arc::clone(&frame_mutex);
    let listener2 = Arc::clone(&listener);
    reset.set_callback(move |_| {
        listener2.service.reset_settings();
        load_settings(&frame2);
    });
    f.fixed(&reset, reset.measure_label().0 + 16);
    f.end();
    frame.fixed(&f, reset.measure_label().1 + 8);
    format_button(&mut reset);
}



pub fn info(frame: MenuFrame) {
    frame.begin();
    frame.draw(|f| {
        draw::set_draw_color(Theme::COLOR);
        draw::set_font(enums::Font::Helvetica, 12);
        draw::draw_text("Açık kaynaklı PvP makrosu\nİstek ve görüşleriniz için:", f.x() + 8, f.y() + 20);
        let mut email = String::from("iletisim");
        email.push_str("@metw.cc");
        draw::draw_text(&email[..], f.x() + 8, f.y() + 50);

        draw::draw_text(&format!("{} - v{}", crate::NAME, crate::VERSION)[..], f.x() + 8, f.y() + f.h() - 8);
    });

    let mut pack = group::Pack::default();
    pack.set_spacing(12);

    let _ = frame::Frame::default().with_size(0, 48);

    // links
    for (label, asset, url) in [
        ("GitHub", "info/git.svg", "https://github.com/metwse/pvp-macro"),
        ("Discord", "info/discord.svg", "https://metw.cc/discord"),
    ] {
        let mut btn = button::Button::default().with_size(48, 48).with_label(label);
        let mut image = get_svg(asset);
        image.scale(24, 24, true, true);
        btn.set_image(Some(image));
        btn.set_frame(enums::FrameType::FlatBox);
        btn_cursor(&mut btn);

        btn.set_callback(|_| {
            let _ = webbrowser::open(url);
        });
    }

    pack.end();
    frame.end();
}


pub fn run(frame: MenuFrame, listener: Arc<Listener>) {
    frame.begin();
    let _ = frame::Frame::default().with_label("pvp macro çalışıyor");
    listener.start().unwrap_or(());
    frame.end();
}


pub fn keybindings(frame: MenuFrame) {
    frame.begin();
    frame.set_type(group::FlexType::Column);

    enum KeyType {
        Num,
        All,
    }

    fn keybinding(label: &str, buttons: &[(KeyType, &str)]) -> group::Flex {
        let mut flex = group::Flex::default();

        let frame = frame::Frame::default().with_label(label);
        flex.fixed(&frame, frame.measure_label().0 + 8);
        let _ = frame::Frame::default();
        for (btn_type, default_key) in buttons {
            let mut btn = button::Button::default().with_label(default_key);
            flex.fixed(&btn, btn.measure_label().0 + 16);
            format_button(&mut btn);
            btn_cursor(&mut btn);
            let _ = match btn_type {
                KeyType::Num => {
                    true
                },
                KeyType::All => {
                    true
                },
            };
        }

        flex.end();
        flex
    }
    
    frame.fixed(&keybinding("Başlat", &[(KeyType::All, "CTRL")]), 24);
    frame.fixed(&frame::Frame::default(), 4);
    frame.fixed(&keybinding("Kılıç eli", &[(KeyType::Num, "1")]), 24);
    frame.fixed(&keybinding("Olta eli", &[(KeyType::Num, "2")]), 24);
    frame.fixed(&frame::Frame::default(), 4);

    for (i, key) in ["z", "x", "c", "v"].iter().enumerate() {
        frame.fixed(
            &keybinding(&format!("Özel {}", i + 1)[..], 
                &[
                (KeyType::Num, &(i + 1).to_string()[..]),
                (KeyType::All, key),
                ]), 
            24);
    }

    frame.end();
}



pub fn metw() {
    let _ = webbrowser::open("https://metw.cc/a/pvp-macro");
}
