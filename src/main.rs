mod image;
use image::{frgb, Image};
use std::fs::File;
use std::io;
use std::path::Path;
#[cfg(feature = "png")]
extern crate png;

fn main() -> io::Result<()> {
    let mut img = Image::empty(3, 2);
    img.set(0, 0, frgb(1.0, 0.0, 0.0));
    img.set(1, 0, frgb(0.0, 1.0, 0.0));
    img.set(2, 0, frgb(0.0, 0.0, 1.0));
    img.set(0, 1, frgb(0.0, 0.0, 0.0));
    img.set(1, 1, frgb(0.5, 0.5, 0.5));
    img.set(2, 1, frgb(1.0, 1.0, 1.0));

    #[cfg(feature = "png")]
    {
        let path = Path::new("out/image.png");
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);
        if let Err(e) = img.write_png(&mut w) {
            eprintln!("Failed to write image: {}", e);
        }
    }
    #[cfg(not(feature = "png"))]
    {
        let path = Path::new("out/image.ppm");
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);
        img.write_ppm(&mut w)?;
    }
    Ok(())
}
