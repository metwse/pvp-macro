use fltk::{prelude::*, *};

use super::{ theme, Theme, util::* };
use webbrowser;

pub type MenuFrame<'a> = &'a mut group::Flex;

pub fn settings(frame: MenuFrame) {
    #[derive(Debug)]
    enum In {
        F (input::FloatInput),
        I (input::IntInput),
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

    for (text, input) in 
        [
            ("Kılıç CPS", In::F(input::FloatInput::default())),
            ("Olta atma sayısı", In::I(input::IntInput::default())),
            ("Olta başına atma süresi", In::F(input::FloatInput::default())),
            ("Rastgelelik yüzdesi", In::F(input::FloatInput::default())),
        ] 
    {
        match input {
            In::F(mut input) => input_num_field(frame, String::from(text), &mut input),
            In::I(mut input) => input_num_field(frame, String::from(text), &mut input),
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



pub fn run(frame: MenuFrame) {
    frame.begin();
    let _ = frame::Frame::default().with_label("Not implemented yet");
    frame.end();
}



pub fn metw() {
    let _ = webbrowser::open("https://metw.cc/a/pvp-macro");
}
