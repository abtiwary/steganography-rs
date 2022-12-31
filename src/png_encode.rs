#[allow(non_camel_case_types)]

use std::fs::File;
use std::io::BufWriter;

use crate::encoder::Encoder;

use anyhow::{Context, Result};

#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct PngEncode {
    png_file_path: String,
    enc_png_file_path: String,
}

impl PngEncode {
    pub fn new(path: &str, out_path: &str) -> Self {
        PngEncode { 
            png_file_path: path.to_string(), 
            enc_png_file_path: out_path.to_string()
        }
    }

    pub fn get_header_info_from_png(&self, png_file_path: &str) -> Result<(
        png::ColorType,
        png::BitDepth,
        Option<png::ScaledFloat>,
        Option<png::SourceChromaticities>
    )> {
        let png_file = File::open(&png_file_path).context("error opening the specified file")?;
        let mut png_decoder = png::Decoder::new(&png_file);
        let header_info = png_decoder.read_header_info()?;
        
        let ct = header_info.color_type;
        let bd = header_info.bit_depth;

        let sg = header_info.source_gamma;
        let sc = header_info.source_chromaticities;
        
        Ok((ct, bd, sg, sc))
    }

    pub fn encode_message(&self, message: &str) -> Result<()> {
        let header_info = self.get_header_info_from_png(&self.png_file_path)?;

        let png_file = File::open(&self.png_file_path).context("error opening the specified file")?;
        let png_decoder = png::Decoder::new(&png_file);

        let mut reader = png_decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let mut bytes = &mut buf[..info.buffer_size()];

        let mut steg_encoder = Encoder::new(&mut bytes, message.as_bytes())?;
        let enc_size = &steg_encoder.encode()?;
        println!("{:?}", enc_size);

        let enc_png_file = File::create(&self.enc_png_file_path).context("error creating the specified file")?;
        let ref mut w = BufWriter::new(enc_png_file);
        let mut encoder = png::Encoder::new(w, info.width, info.height);
        encoder.set_color(info.color_type);
        encoder.set_depth(info.bit_depth);

        let source_gamma = header_info.2;
        if source_gamma.is_some() {
            encoder.set_source_gamma(source_gamma.unwrap());
        }
        
        let source_chromaticities = header_info.3;
        if source_chromaticities.is_some() {
            encoder.set_source_chromaticities(source_chromaticities.unwrap());
        }

        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&bytes).unwrap();

        Ok(())
    }
}
