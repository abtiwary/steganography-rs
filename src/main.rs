use::std::path::Path;

use stegano_rs::png_encode::PngEncode;
use stegano_rs::png_decode::PngDecode;

use anyhow::{Result};
use clap::{Arg, Command};
use slog::*;
use ulid::Ulid;

fn main() -> Result<()> {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let logger = slog::Logger::root(
        slog_term::FullFormat::new(decorator).build().fuse(), 
        o!()
    );

    let matches = Command::new("Encode a message in a PNG file")
        .version("0.1")
        .author("Ab Tiwary")
        .about("A Steganography app for pedagogical purposes")
        .arg(Arg::new("input-file"))
            .short_flag('i')
            .long_flag("input-file")
        .get_matches();

    if let Some(png_file) = matches.get_one::<String>("input-file") {
        info!(logger, "Received an input file from the commandline args: {}", &png_file);

        let png_file_path = Path::new(&png_file);

        let png_file_dir = png_file_path.parent().unwrap().to_str().unwrap();
        let png_file_name = png_file_path.file_name().unwrap().to_str().unwrap();

        let ulid = Ulid::new();
        let out_file = format!("{}_{}", ulid.to_string(), png_file_name);
        let out_file_path = Path::new(png_file_dir).join(out_file);
        let out_file_path = out_file_path.to_str().unwrap();
        info!(logger, "going to write this file: {}", &out_file_path);

        let png_steg_encode = PngEncode::new(&png_file, &out_file_path);
        let _ = png_steg_encode.encode_message("this is some secret message to encode").unwrap();

        let png_steg_decode = PngDecode::new(&out_file_path);
        let _ = png_steg_decode.decode_message().unwrap();


    } else {
        panic!("expected a valid input file")
    }

    println!("done");

    Ok(())
    
}
