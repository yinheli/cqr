use anyhow::Error;
use clap::Parser;
use image::ImageBuffer;
use quircs::Quirc;

mod cli;

fn main() {
    let args = cli::Args::parse();

    match args.command {
        cli::Command::Decode { value } => {
            if let Err(e) = decode(value) {
                println!("error: {:?}", e)
            }
        }
    }
}

fn decode(value: String) -> Result<(), Error> {
    let img;
    if value.starts_with("http") && value.contains("://") {
        let response = reqwest::blocking::get(value)?;
        img = image::load_from_memory(&response.bytes()?)?;
    } else {
        img = image::open(value)?;
    }

    let img = img.into_luma8();
    let mut wrap = ImageBuffer::from_fn(img.width() + 10, img.height() + 10, |_x, _y| {
        image::Luma([0xff])
    });

    image::imageops::overlay(&mut wrap, &img, 5, 5);

    let mut decoder = Quirc::default();
    let codes = decoder.identify(wrap.width() as usize, wrap.height() as usize, &wrap);

    for code in codes.into_iter().flatten() {
        match code.decode() {
            Ok(data) => {
                let v = String::from_utf8_lossy(&data.payload);
                println!("{}", v);
            }
            Err(e) => {
                println!("decode failed: {:?}", e);
            }
        }
    }

    Ok(())
}
