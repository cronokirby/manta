mod geometry;
mod image;
mod tracer;

use std::fs::File;
use std::io;
use std::path::Path;
#[cfg(feature = "png")]
extern crate png;

fn main() -> io::Result<()> {
    let width = 400;

    let img = tracer::trace(width);

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
