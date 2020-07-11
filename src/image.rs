#[cfg(feature = "png")]
use png;
use std::io;

/// Represents a pixel in RGBA, in floating point terms.
///
/// This is more useful for ray tracing itself, and can easily be converted to the final
/// image pixel type.
#[derive(Clone, Copy, Debug)]
pub struct FRGBA {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

impl FRGBA {
    pub fn lerp(&self, t: f64, that: FRGBA) -> FRGBA {
        let lrp = |a, b| a - t * (a - b);
        frgb(
            lrp(self.r, that.r),
            lrp(self.g, that.g),
            lrp(self.b, that.b),
        )
    }
}

fn clamp(f: f64) -> f64 {
    if f > 1.0 {
        1.0
    } else if f < 0.0 {
        0.0
    } else {
        f
    }
}

/// Create a new FRGBA color with full opacity
pub fn frgb(r: f64, g: f64, b: f64) -> FRGBA {
    FRGBA {
        r: clamp(r),
        g: clamp(g),
        b: clamp(b),
        a: 1.0,
    }
}

/// Represents an RGBA color / pixel
///
/// This is our main representation of colors, and a pretty simple struct as well
#[derive(Clone, Copy, Debug)]
pub struct RGBA {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

fn convert_component(c: f64) -> u8 {
    (c * 255.999) as u8
}

impl From<FRGBA> for RGBA {
    fn from(color: FRGBA) -> Self {
        RGBA {
            r: convert_component(color.r),
            g: convert_component(color.g),
            b: convert_component(color.b),
            a: convert_component(color.a),
        }
    }
}

/// Represents an Image.
///
/// In our conception, an image is simply a 2D collection of RGBA pixels.
/// Images are the canvas on which we draw data, and are the final step before we
/// output them to a file
#[derive(Clone, Debug)]
pub struct Image {
    // We could have an array of pixels instead, but seperating out the components is
    // usually a bit friendlier towards writing to a file. Namely, the PNG crate we use
    // expects a contiguous array of bytes
    data: Vec<u8>,
    // The width of the image
    width: usize,
    // The height of the image
    height: usize,
}

impl Image {
    /// Create an empty image.
    ///
    /// Visually, this will be filled with transparent black pixels
    pub fn empty(width: usize, height: usize) -> Self {
        let data = vec![0; (width * height) << 2];
        Image {
            data,
            width,
            height,
        }
    }

    /// Set a single pixel in the image
    ///
    /// The coordinate system varies in the standard way, with (x: 0, y: 0) being
    /// the top left corner, and (width - 1, height - 1) being the bottom right corner.
    ///
    /// Trying to set an out of bounds pixel will panic, but not with the nicest error
    /// message.
    pub fn set<C: Into<RGBA>>(&mut self, x: usize, y: usize, color: C) {
        let color = color.into();
        let start = (self.width * y + x) << 2;
        self.data[start] = color.r;
        self.data[start + 1] = color.g;
        self.data[start + 2] = color.b;
        self.data[start + 3] = color.a;
    }

    /// Write this image in PPM format to some buffer
    ///
    /// The PPM format is simple, but has no compression, and isn't commonly used.
    #[allow(dead_code)]
    pub fn write_ppm<W: io::Write>(&mut self, mut w: W) -> io::Result<()> {
        writeln!(&mut w, "P6")?;
        writeln!(&mut w, "{} {} 255", self.width, self.height)?;
        let mut no_alpha = Vec::with_capacity(self.width * self.height * 3);
        for i in 0..self.data.len() {
            if ((i + 1) & 0b11) != 0 {
                no_alpha.push(self.data[i]);
            }
        }
        w.write_all(&no_alpha)
    }

    /// Write this image in PNG format
    ///
    /// The PNG format is lossless, but still uses compression. It's the standard for
    /// faithful image formats.
    #[cfg(feature = "png")]
    pub fn write_png<W: io::Write>(&self, w: W) -> Result<(), png::EncodingError> {
        let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.data)
    }
}
