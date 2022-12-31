#[allow(non_camel_case_types)]

use std::fs::File;

use crate::decoder::Decoder;

use anyhow::{Context, Result};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct PngDecode {
    png_file_path: String,
}

impl PngDecode {
    pub fn new(path: &str) -> Self {
        PngDecode { 
            png_file_path: path.to_string()
        }
    }

    pub fn decode_message(&self) -> Result<Vec<u8>> {
        let png_file = File::open(&self.png_file_path).context("error opening the specified file")?;
        let png_decoder = png::Decoder::new(&png_file);

        let mut reader = png_decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let bytes = &mut buf[..info.buffer_size()];

        let mut steg_decoder = Decoder::new(&bytes)?;
        let decoded_message = steg_decoder.decode()?;

        println!("{:?}", String::from_utf8(decoded_message.clone()).unwrap());

        Ok(decoded_message)
    }
}
