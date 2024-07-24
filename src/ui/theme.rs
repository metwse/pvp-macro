use fltk::{
    prelude::*,
    *,
    enums::Color,
};

pub struct Theme;
impl Theme {
    pub const NAVBAR_BG: Color = Color::from_hex(0x3C4043);
    pub const NAVBAR_QUIT_BG: Color = Color::from_hex(0xDF0135);
    pub const BG_1: Color = Color::from_hex(0x5C6063);
    pub const BG_2: Color = Color::from_hex(0x4C5053);

    pub const COLOR: Color = Color::from_hex(0xF0F0F0);
}


pub fn format_input(input: &mut dyn InputExt) {
    input.set_frame(enums::FrameType::FlatBox);
}

pub fn format_button(btn: &mut dyn ButtonExt) {
    btn.set_frame(enums::FrameType::FlatBox);
    btn.set_color(Theme::BG_2);
}
