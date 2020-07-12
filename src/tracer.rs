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

fn ray_color(ray: &Ray, world: &dyn Hittable) -> FRGBA {
    if let Some(rec) = world.hit(ray, 0.0, f64::INFINITY) {
        let n = rec.normal;
        return frgb((n.x + 1.0) / 2.0, (n.y + 1.0) / 2.0, (n.z + 1.0) / 2.0);
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

    let mut world = HittableList::new();
    world.add(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
    });
    world.add(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
    });

    for y in 0..height {
        for x in 0..width {
            let u = x as f64 / (width - 1) as f64;
            let v = 1.0 - y as f64 / (height - 1) as f64;
            let direction = lower_left + horizontal * u + vertical * v - origin;
            let ray = Ray { origin, direction };
            image.set(x, y, ray_color(&ray, &world));
        }
        println!("line {} / {}", y + 1, height);
    }
    image
}
