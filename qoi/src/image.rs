use byteorder::{BigEndian, ReadBytesExt};

use std::io;
use std::io::Read;

use super::decoder::Decoder;

pub struct Header {
    pub width: usize,
    pub height: usize,
    pub channels: u8,
    pub colorspace: u8,
}

impl Header {
    pub fn channels_name(&self) -> String {
        match self.channels {
            3 => "RGB".into(),
            4 => "RGBA".into(),
            _ => "unknown".into(),
        }
    }

    pub fn colorspace_name(&self) -> String {
        match self.colorspace {
            0 => "sRGB with linear alpha".into(),
            1 => "all channels linear".into(),
            _ => "unknown".into(),
        }
    }

    pub fn from_reader(reader: &mut impl Read) -> Result<Self, io::Error> {
        let mut magic = [0u8; 4];
        reader.read_exact(&mut magic)?;
        if magic != [0x71, 0x6f, 0x69, 0x66] {
            return Err(io::Error::new::<&'static str>(
                io::ErrorKind::Other,
                "bad magic",
            ));
        }

        let width = reader.read_u32::<BigEndian>()?;
        let height = reader.read_u32::<BigEndian>()?;
        let channels = reader.read_u8()?;
        let colorspace = reader.read_u8()?;

        Ok(Header {
            width: width as usize,
            height: height as usize,
            channels,
            colorspace,
        })
    }
}

pub struct Image {
    pub header: Header,
    // RGBA
    pub buffer: Vec<u8>,
}

impl Image {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, io::Error> {
        let header = Header::from_reader(reader)?;

        let mut buffer = vec![0u8; header.width * header.height * 4];

        let mut decoder = Decoder::new(&header);
        for row in 0..header.height {
            let start = (row * header.width * 4) as usize;
            let end = ((row + 1) * header.width * 4) as usize;

            decoder.decode_row(reader, &mut buffer[start..end])?;
        }

        Ok(Image { header, buffer })
    }
}
