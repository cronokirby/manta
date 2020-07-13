mod geometry;
mod image;
mod tracer;

use std::fs::File;
use std::io;
use std::path::Path;
#[cfg(feature = "png")]
extern crate png;
extern crate fastrand;

fn main() -> io::Result<()> {
    let width = 400;
    let height: usize = 400 / 16 * 9;

    let img = tracer::trace(width, height);

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
