use fltk::{prelude::*, *};
use super::Theme;

pub fn start(title: &str, default: &str, current: &str) {
    let mut win = window::Window::default().with_size(256, 128).center_screen();

    let mut flex = group::Flex::default().size_of_parent();
    flex.set_type(group::FlexType::Column);
    flex.set_margin(16);

    let mut title = frame::Frame::default().with_label(title);
    flex.fixed(&title, title.measure_label().1);
    title.set_label_size(20);

    let _default_frame = frame::Frame::default().with_label(&format!("varsayılan: {}", default)[..]);

    let mut group = group::Flex::default();
    let _ = frame::Frame::default();
    let mut current_frame = frame::Frame::default().with_label(current);
    let _ = frame::Frame::default();
    current_frame.set_frame(enums::FrameType::FlatBox);
    current_frame.set_label_size(18);
    current_frame.set_color(Theme::BG_2);
    group.set_color(enums::Color::Blue);
    flex.fixed(&group, current_frame.measure_label().1 + 16);

    group.end();


    win.end();
    win.show();
}
