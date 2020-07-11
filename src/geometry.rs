use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A type alias for vector, emphasizing their use as points in 3D space
type Point3 = Vec3;

impl Vec3 {
    /// The squared length of this vector
    pub fn len2(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// The length of this vector
    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, that: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + that.x,
            y: self.y + that.y,
            z: self.z + that.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, that: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - that.x,
            y: self.y - that.y,
            z: self.z - that.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, that: Vec3) {
        self.x -= that.x;
        self.y -= that.y;
        self.z -= that.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, scale: f64) -> Vec3 {
        Vec3 {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scale: f64) {
        self.x *= scale;
        self.y *= scale;
        self.z *= scale;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        self * -1.0
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, scale: f64) -> Vec3 {
        // Unsure if this poses numerical problems? It involves less code though
        self * (1.0 / scale)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scale: f64) {
        *self *= 1.0 / scale;
    }
}
