mod image;
use image::{frgb, Image};
use std::fs::File;
use std::io;
use std::path::Path;
#[cfg(feature = "png")]
extern crate png;

fn main() -> io::Result<()> {
    let width = 400;
    let height = 200;

    let mut img = Image::empty(width, height);
    for y in 0..height {
        for x in 0..width {
            let color = frgb(
                x as f64 / (width - 1) as f64,
                y as f64 / (height - 1) as f64,
                0.25,
            );
            img.set(x, y, color);
        }
    }

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
