use fltk::{prelude::*, *};

use crate::keyboard::listener::MacroListener;

use std::sync::Arc;

use super::{ 
    theme::{self, format_button}, Theme,
    util::* 
};
use webbrowser;


pub type MenuFrame<'a> = &'a mut group::Flex;

pub fn settings(frame: MenuFrame) {
    #[derive(Debug)]
    enum In {
        F (f64),
        I (i32),
    }

    frame.set_type(group::FlexType::Column);

    fn input_num_field(frame: MenuFrame, text: String, input: &mut impl InputExt) {
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
    }

    for (text, input_type) in 
        [
            ("Kılıç CPS", In::F(13.0)),
            ("Kılıç vurma sayısı", In::I(13)),
            ("Olta atma sayısı", In::I(5)),
            ("Olta başına atma süresi", In::F(0.2)),
            ("Rastgelelik yüzdesi", In::F(20.0)),
        ] 
    {
        match input_type {
            In::F(default) => {
                let mut input = input::FloatInput::default();
                input_num_field(frame, String::from(text), &mut input);
                input.set_value(&default.to_string()[..]);
            },
            In::I(default) => {
                let mut input = input::IntInput::default();
                input_num_field(frame, String::from(text), &mut input);
                input.set_value(&default.to_string()[..]);
            },
        }
    }
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


pub fn run(frame: MenuFrame, listener: Arc<MacroListener>) {
    frame.begin();
    let _ = frame::Frame::default().with_label("Not implemented yet");
    println!("{}", (*listener).is_listening());
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
