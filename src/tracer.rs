use crate::geometry::{Point3, Vec3};
use crate::image::{frgb, Image, FRGBA};
use fastrand;
use std::f64::consts::PI;

fn rand_range(min: f64, max: f64) -> f64 {
    min + (max - min) * fastrand::f64()
}

fn unit_rand() -> Vec3 {
    let a = rand_range(0.0, 2.0 * PI);
    let z = rand_range(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();
    Vec3::new(r * a.cos(), r * a.sin(), z)
}

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

/// Represents a camera that views the scene, and gives us a viewpoint from which to trace
///
/// The camera is the starting point for ray-tracing, and encodes concerns like the image
/// size, and what we can see, etc.
#[derive(Copy, Clone, Debug)]
struct Camera {
    /// We store the aspect ratio, because it's convenient, even though it can be
    /// derived from other properties.
    aspect: f64,
    /// The origin, which should be 0, 0, 0
    origin: Point3,
    /// The lower left point of the image
    lower_left: Point3,
    /// A vector bringing us across the width of the image
    horizontal: Vec3,
    /// A vector bringing us across the height of the image
    vertical: Vec3,
}

const ASPECT: f64 = 16.0 / 9.0;
const VIEW_HEIGHT: f64 = 2.0;
const VIEW_WIDTH: f64 = ASPECT * VIEW_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

impl Camera {
    fn new() -> Self {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(VIEW_WIDTH, 0.0, 0.0);
        let vertical = Vec3::new(0.0, VIEW_HEIGHT, 0.0);
        let lower_left =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

        Camera {
            aspect: ASPECT,
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}

/// Represents the information we have after hitting a certain point.
#[derive(Copy, Clone, Debug)]
struct HitRecord {
    /// The point we hit
    p: Point3,
    /// The parameter to the ray equation at this point
    t: f64,
    /// The normal of the surface at the point we hit
    normal: Vec3,
    /// Whether or not the normal is pointing outwards
    outwards: bool,
}

impl HitRecord {
    fn new(t: f64, p: Vec3, ray: &Ray, out_normal: Vec3) -> Self {
        let outwards = ray.direction.dot(&out_normal) < 0.0;
        let normal = if outwards { out_normal } else { -out_normal };
        HitRecord {
            p,
            t,
            normal,
            outwards,
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

/// Represents a list of hittable objects
struct HittableList {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        HittableList {
            hittables: Vec::new(),
        }
    }

    fn add<H: Hittable + 'static>(&mut self, hittable: H) {
        self.hittables.push(Box::new(hittable));
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut res: Option<HitRecord> = None;
        let mut closest_t = t_max;
        for obj in &self.hittables {
            if let Some(rec) = obj.hit(ray, t_min, closest_t) {
                res = Some(rec);
                // Always lower, since t_min <= t < t_max
                closest_t = rec.t;
            }
        }
        res
    }
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len2();
        let half_b = oc.dot(&ray.direction);
        let c = oc.len2() - self.radius * self.radius;
        let discrim = half_b * half_b - a * c;

        if discrim < 0.0 {
            None
        } else {
            let root = discrim.sqrt();
            let get_record = |solution| {
                let t = solution;
                let p = ray.at(t);
                let normal = (p - self.center) / self.radius;
                HitRecord::new(t, p, ray, normal)
            };
            let valid_range = t_min..t_max;
            let solution1 = (-half_b - root) / a;
            let solution2 = (-half_b + root) / a;
            if valid_range.contains(&solution1) {
                Some(get_record(solution1))
            } else if valid_range.contains(&solution2) {
                Some(get_record(solution2))
            } else {
                None
            }
        }
    }
}

fn ray_color(mut ray: Ray, world: &dyn Hittable, depth: i32) -> FRGBA {
    let mut dampening = 1.0;
    for _ in 0..depth {
        if let Some(rec) = world.hit(&ray, 0.0001, f64::INFINITY) {
            dampening *= 0.5;
            let target = rec.p + rec.normal + unit_rand();
            ray = Ray {
                origin: rec.p,
                direction: target,
            };
        } else {
            let unit = ray.direction.normalize();
            let t = 0.5 * (unit.y + 1.0);
            let base = frgb(1.0, 1.0, 1.0).lerp(t, frgb(0.5, 0.7, 1.0));
            return frgb(base.r * dampening, base.g * dampening, base.b * dampening);
        }
    }
    frgb(0.0, 0.0, 0.0)
}

/// A struct allowing us to add color samples, and end up with a final mixed color
#[derive(Copy, Clone, Debug)]
struct SampledColor {
    samples: u32,
    acc: FRGBA,
}

impl SampledColor {
    fn empty() -> Self {
        SampledColor {
            samples: 0,
            acc: FRGBA {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        }
    }

    fn add(&mut self, sample: FRGBA) {
        self.samples += 1;
        self.acc = FRGBA {
            r: self.acc.r + sample.r,
            g: self.acc.g + sample.g,
            b: self.acc.b + sample.b,
            a: self.acc.a + sample.a,
        }
    }

    fn result(&self) -> FRGBA {
        let total = self.samples as f64;
        FRGBA {
            r: (self.acc.r / total).sqrt(),
            g: (self.acc.g / total).sqrt(),
            b: (self.acc.b / total).sqrt(),
            a: self.acc.a / total,
        }
    }
}

const SAMPLES_PER_PIXEL: i32 = 50;
const MAX_DEPTH: i32 = 50;

pub fn trace(width: usize) -> Image {
    let height = ((width as f64) / ASPECT) as usize;
    let mut image = Image::empty(width, height);

    let mut world = HittableList::new();
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    let camera = Camera::new();

    for y in 0..height {
        for x in 0..width {
            let mut sampled = SampledColor::empty();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (fastrand::f64() + x as f64) / (width - 1) as f64;
                let v = 1.0 - (y as f64 - fastrand::f64()) / (height - 1) as f64;
                let ray = camera.get_ray(u, v);
                sampled.add(ray_color(ray, &world, MAX_DEPTH));
            }
            image.set(x, y, sampled.result());
        }
        println!("line {} / {}", y + 1, height);
    }
    image
}
