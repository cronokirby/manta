use std::ops;

/// A representation of a an element of R^3.
///
/// Mathematically, these are elements of that vector field, and thus can
/// be scaled by real numbers (here represented as f64s), added together,
/// along with other operations that are more specific to R.
///
/// Geometrically, these are generally viewed as a directed arrow equipped with a length,
/// and a lot of the basic geometric operations will make use of their properties.
///
/// The coordinates of the vector are exposed, as not much can be gained from hiding
/// them.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new vector from its components
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    /// The dot product between two vectors
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// The length of this vector
    pub fn len(self) -> f64 {
        self.len2().sqrt()
    }

    /// The squared length of this vector?
    ///
    /// Why care about the squared length? It's slightly faster to compute,
    /// and can often be as useful as the distance itself.
    pub fn len2(self) -> f64 {
        self.dot(self)
    }
}

// PERFORMANCE NOTE: For all of the operations, we work on moves / copies of the vector.
// In theory, the compiler should be able to avoid copying, and instead do the more
// efficient pass by reference.

// vec + vec
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// vec += vec
impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

// vec * real
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Self {
        Vec3 {
            x: scale * self.x,
            y: scale * self.y,
            z: scale * self.z,
        }
    }
}

// vec *= real
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scale: f64) {
        *self = *self * scale
    }
}

// real * vec
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        vector * self
    }
}

// vec / real
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scale: f64) -> Self {
        // NOTE: could this pose numerical problems? It saves code, at least
        (1.0 / scale) * self
    }
}

// -vec
impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        -1.0 * self
    }
}

// vec - vec
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

// vec -= vec
impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other
    }
}

/// Represents a single point in 3D space.
///
/// At a first glance, this might seem to be like the vector struct. Indeed,
/// they're isomorphic, and have the same structure. But they represent different
/// geometric concepts. A vector represents a direction along with a length, and
/// has a dimension, but a point is dimensionless.
///
/// The operations around points often involve vectors instead of points, which
/// is another difference. Basically, we can only do interesting things to points
/// by the means of a vector, whereas a vector is "self-sufficient" in some sense.
#[derive(Copy, Clone, Debug, PartialEq)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    /// Create a new point from its components
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point3 { x, y, z }
    }

    /// The point at the origin of the 3D space
    pub fn origin() -> Self {
        Point3::new(0.0, 0.0, 0.0)
    }

    /// The distance from this point to another
    pub fn dist(self, to: Point3) -> f64 {
        (self - to).len()
    }

    /// The squared distance from this point to another
    pub fn dist2(self, to: Point3) -> f64 {
        (self - to).len2()
    }
}

impl ops::Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vec3) -> Self {
        Point3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, other: Vec3) {
        *self = *self + other
    }
}

// Returns the vector that brings another point to this one
impl ops::Sub<Point3> for Point3 {
    type Output = Vec3;

    fn sub(self, other: Point3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self {
        Point3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, other: Vec3) {
        *self = *self - other
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec3_add_assign() {
        let mut x = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(x, Vec3::new(0.0, 0.0, 0.0));
        x += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(x, Vec3::new(1.0, 2.0, 3.0));
        x += Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(x, Vec3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0) + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3::new(0.0, 0.0, 0.0) - Vec3::new(1.0, 1.0, 1.0),
            -Vec3::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn test_vec3_mul() {
        let x = Vec3::new(1.0, 2.0, 3.0);
        let y = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(x * 3.0, y);
        assert_eq!(3.0 * x, y);
    }

    #[test]
    fn test_vec3_div() {
        let x = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(x / 2.0, Vec3::new(1.0, 1.0, 1.0));
    }
}
