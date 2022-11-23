use std::ops::{Add, Mul, Sub};

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    /// The result of subtracting two 3D vectors is another 3D vector.
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        // Vectors are added component-wise
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn some_lengths() {
        assert_eq!(Vec3::new(1.0, 2.0, 2.0).length(), 3.0);
        assert_eq!(Vec3::new(2.0, 10.0, 11.0).length(), 15.0);
    }

    // Create some constants so that we can reuse the values
    // across our addition and subtraction tests.
    const A: Vec3 = Vec3::new(1.0, 2.0, 4.0);
    const B: Vec3 = Vec3::new(5.0, 3.0, 7.0);
    const C: Vec3 = Vec3::new(6.0, 5.0, 11.0);

    #[test]
    fn an_addition() {
        assert_eq!(A + B, C);
    }

    #[test]
    fn a_subtraction() {
        assert_eq!(C - B, A);
    }

    #[test]
    fn a_multiplication() {
        assert_eq!(2.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(2.0, 4.0, 6.0));
    }
}
