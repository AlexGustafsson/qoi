use byteorder::ReadBytesExt;

use std::io;
use std::io::Read;

use super::color::Color;
use super::image::Header;

pub struct Decoder {
    width: usize,
    lookup: [u32; 64],
    previous_color: Color,
    remaining_run: u8,
}

impl Decoder {
    pub fn new(header: &Header) -> Self {
        Decoder {
            width: header.width,
            lookup: [0u32; 64],
            previous_color: Color::from_value(0x000000FF),
            remaining_run: 0,
        }
    }

    fn hash(r: u8, g: u8, b: u8, a: u8) -> u8 {
        // Can't panic due to only the lower 8 bits being used (& 0xFF)
        (((r as u32 * 3 + g as u32 * 5 + b as u32 * 7 + a as u32 * 11) % 64) & 0xFF)
            .try_into()
            .unwrap()
    }

    pub fn decode_row(
        &mut self,
        reader: &mut impl Read,
        buffer: &mut [u8],
    ) -> Result<(), io::Error> {
        let mut i = 0usize;

        while self.remaining_run > 1 && i < self.width {
            buffer[i * 4 + 0] = self.previous_color.r;
            buffer[i * 4 + 1] = self.previous_color.g;
            buffer[i * 4 + 2] = self.previous_color.b;
            buffer[i * 4 + 3] = self.previous_color.a;
            self.remaining_run -= 1;
            i += 1;
        }

        while i < self.width {
            let tag8 = reader.read_u8()?;
            let tag2 = (tag8 & 0xC0) >> 6;

            if tag8 == 0xFE {
                // QOI_OP_RGB
                let r = reader.read_u8()?;
                let g = reader.read_u8()?;
                let b = reader.read_u8()?;
                let a = self.previous_color.a;
                let color = Color::pack_components(r, g, b, a);

                self.lookup[Decoder::hash(r, g, b, a) as usize] = color;
                self.previous_color = Color::from_components(r, g, b, a);

                buffer[i * 4 + 0] = r;
                buffer[i * 4 + 1] = g;
                buffer[i * 4 + 2] = b;
                buffer[i * 4 + 3] = a;
                i += 1;
            } else if tag8 == 0xFF {
                // QOI_OP_RGBA
                let r = reader.read_u8()?;
                let g = reader.read_u8()?;
                let b = reader.read_u8()?;
                let a = reader.read_u8()?;
                let color = Color::pack_components(r, g, b, a);

                self.lookup[Decoder::hash(r, g, b, a) as usize] = color;
                self.previous_color = Color::from_components(r, g, b, a);

                buffer[i * 4 + 0] = r;
                buffer[i * 4 + 1] = g;
                buffer[i * 4 + 2] = b;
                buffer[i * 4 + 3] = a;
                i += 1;
            } else if tag2 == 0b00 {
                // QOI_OP_INDEX
                let index = tag8 & 0x3F;
                if index > 63 {
                    return Err(io::Error::new::<String>(
                        io::ErrorKind::Other,
                        format!("illegal QOI_OP_INDEX index {}", index),
                    ));
                }

                let color = self.lookup[index as usize];
                self.previous_color = Color::from_value(color);

                buffer[i * 4 + 0] = self.previous_color.r;
                buffer[i * 4 + 1] = self.previous_color.g;
                buffer[i * 4 + 2] = self.previous_color.b;
                buffer[i * 4 + 3] = self.previous_color.a;
                i += 1;
            } else if tag2 == 0b01 {
                // QOI_OP_DIFF
                let r = u8::wrapping_sub(
                    u8::wrapping_add(self.previous_color.r, (tag8 >> 4) & 0x03),
                    2,
                );
                let g = u8::wrapping_sub(
                    u8::wrapping_add(self.previous_color.g, (tag8 >> 2) & 0x03),
                    2,
                );
                let b = u8::wrapping_sub(u8::wrapping_add(self.previous_color.b, tag8 & 0x03), 2);
                let a = self.previous_color.a;
                let color = Color::pack_components(r, g, b, a);

                self.lookup[Decoder::hash(r, g, b, a) as usize] = color;
                self.previous_color = Color::from_components(r, g, b, a);

                buffer[i * 4 + 0] = r;
                buffer[i * 4 + 1] = g;
                buffer[i * 4 + 2] = b;
                buffer[i * 4 + 3] = a;
                i += 1;
            } else if tag2 == 0b10 {
                let vg = (tag8 & 0x3f).wrapping_sub(32);

                // QOI_OP_LUMA
                let d = reader.read_u8()?;
                let r = self
                    .previous_color
                    .r
                    .wrapping_add(vg.wrapping_sub(8).wrapping_add((d >> 4) & 0x0f));
                let g = self.previous_color.g.wrapping_add(vg);
                let b = self
                    .previous_color
                    .b
                    .wrapping_add(vg.wrapping_sub(8).wrapping_add(d & 0x0f));
                let a = self.previous_color.a;
                let color = Color::pack_components(r, g, b, a);

                self.lookup[Decoder::hash(r, g, b, a) as usize] = color;
                self.previous_color = Color::from_components(r, g, b, a);

                buffer[i * 4 + 0] = r;
                buffer[i * 4 + 1] = g;
                buffer[i * 4 + 2] = b;
                buffer[i * 4 + 3] = a;
                i += 1;
            } else if tag2 == 0b11 {
                // QOI_OP_RUN
                let mut length = tag8 & 0x3F;
                if length > 63 {
                    return Err(io::Error::new::<String>(
                        io::ErrorKind::Other,
                        format!("illegal QOI_OP_RUN length {}", length),
                    ));
                }
                // Bias
                length += 1;
                self.remaining_run = length + 1;

                while self.remaining_run > 1 && i < self.width {
                    buffer[i * 4 + 0] = self.previous_color.r;
                    buffer[i * 4 + 1] = self.previous_color.g;
                    buffer[i * 4 + 2] = self.previous_color.b;
                    buffer[i * 4 + 3] = self.previous_color.a;
                    self.remaining_run -= 1;
                    i += 1;
                }
            } else {
                return Err(io::Error::new::<String>(
                    io::ErrorKind::Other,
                    format!("unknown tag {}", tag8),
                ));
            }
        }

        Ok(())
    }
}
