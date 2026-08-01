#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl From<hex_color::HexColor> for Color {
    fn from(value: hex_color::HexColor) -> Self {
        Color::rgb(value.r, value.g, value.b)
    }
}

impl From<Color> for tiny_skia::Color {
    fn from(value: Color) -> Self {
        tiny_skia::Color::from_rgba8(value.r, value.g, value.b, 255)
    }
}

impl<'a> From<Color> for tiny_skia::Paint<'a> {
    fn from(value: Color) -> Self {
        let mut paint: tiny_skia::Paint = tiny_skia::Paint::default();
        paint.set_color(value.into());
        paint.anti_alias = true;
        paint
    }
}
