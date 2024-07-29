use fltk::{prelude::*, *};

pub fn btn_cursor(wid: &mut (impl ButtonExt + WidgetExt + WidgetBase)) {
    let mut win = wid.top_window().unwrap();

    wid.handle(move |_, event: enums::Event| {
        match event {
            enums::Event::Enter => {
                (*win).set_cursor(enums::Cursor::Hand);
                true
            },
            enums::Event::Leave => {
                (*win).set_cursor(enums::Cursor::Arrow);
                true
            },
            _ => false,
        }
    })
}

// gets svg from assets
pub fn get_svg(file: &str) -> image::SvgImage {
    let image_data = std::str::from_utf8(
        crate::ASSETS_DIR.get_file(file)
        .unwrap()
        .contents()
    ).unwrap();
    image::SvgImage::from_data(image_data).unwrap()
}
