#[repr(C, packed)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn from_components(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn from_value(color: u32) -> Color {
        Color {
            r: ((color & 0xFF000000) >> 24).try_into().unwrap(),
            g: ((color & 0x00FF0000) >> 16).try_into().unwrap(),
            b: ((color & 0x0000FF00) >> 8).try_into().unwrap(),
            a: (color & 0x000000FF).try_into().unwrap(),
        }
    }

    pub fn pack_components(r: u8, g: u8, b: u8, a: u8) -> u32 {
        (r as u32) << 24 | (g as u32) << 16 | (b as u32) << 8 | (a as u32)
    }
}
