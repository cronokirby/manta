use crate::geometry::{Point3, Vec3};
use crate::image::{frgb, Image, FRGBA};

/// Represents a ray of light moving along a certain line
#[derive(Clone, Copy, Debug)]
struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Calculate what point the ray will be at after a certain time
    ///
    /// The idea is that the ray traces a line for each number t
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.len2();
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.len2() - self.radius * self.radius;
        let discrim = b * b - 4.0 * a * c;
        discrim > 0.0
    }
}

fn ray_color(ray: &Ray) -> FRGBA {
    let sphere = Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    if sphere.intersects(ray) {
        return frgb(1.0, 0.0, 0.0);
    }
    let unit_dir = ray.direction.normalize();
    let t = 0.5 * (unit_dir.y + 1.0);
    frgb(1.0, 1.0, 1.0).lerp(t, frgb(0.5, 0.7, 1.0))
}

const ASPECT: f64 = 16.0 / 9.0;
const VIEW_HEIGHT: f64 = 2.0;
const VIEW_WIDTH: f64 = ASPECT * VIEW_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

pub fn trace(width: usize) -> Image {
    let height = ((width as f64) / ASPECT) as usize;
    let mut image = Image::empty(width, height);

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(VIEW_WIDTH, 0.0, 0.0);
    let vertical = Vec3::new(0.0, VIEW_HEIGHT, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = 1.0 - y as f64 / (height - 1) as f64;
            let direction = lower_left + horizontal * u + vertical * v - origin;
            let ray = Ray { origin, direction };
            image.set(x, y, ray_color(&ray));
        }
        println!("line {} / {}", y + 1, height);
    }
    image
}
