use fltk::{prelude::*, *};

pub fn btn_cursor(wid: &mut (impl ButtonExt + WidgetExt + WidgetBase)) {

    wid.handle(|_, event: enums::Event| {
        use fltk::{
            window::Window,
            enums::{Cursor, Event}
        };

        let mut win: Window = app::widget_from_id("window").unwrap();
        match event {
            Event::Enter => {
                win.set_cursor(Cursor::Hand);
                true
            },
            Event::Leave => {
                win.set_cursor(Cursor::Arrow);
                true
            },
            _ => false,
        }
    })
}

pub fn get_svg(file: &str) -> image::SvgImage {
    let image_data = std::str::from_utf8(crate::ASSETS_DIR.get_file(file).unwrap().contents()).unwrap();
    image::SvgImage::from_data(image_data).unwrap()
}
