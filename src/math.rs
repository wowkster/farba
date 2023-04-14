/* ===== Vec3 ===== */

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y + self.y * self.z * self.z
    }

    pub fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();

        Vec3::new(self.x / mag, self.y / mag, self.z / mag)
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

/* ===== Vec2 ===== */

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self::new(0.0, 0.0);

    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2::new(rhs.x * self, rhs.y * self)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

/* ==== Mat3 ==== */

/// Represents the 3x3 matrix with the following values:
///
/// | a, b, c |
/// | d, e, f |
/// | g, h, i |
#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
    pub g: f32,
    pub h: f32,
    pub i: f32,
}

impl Mat3 {
    pub fn new(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
        }
    }

    #[rustfmt::skip]
    pub fn rotate_x(angle: f32) -> Mat3 {
        Self {
            a: 1.0, b: 0.0,             c: 0.0,
            d: 0.0, e: f32::cos(angle), f: -f32::sin(angle),
            g: 0.0, h: f32::sin(angle), i: f32::cos(angle),
        }
    }

    #[rustfmt::skip]
    pub fn rotate_y(angle: f32) -> Mat3 {
        Self {
            a: f32::cos(angle),  b: 0.0, c: f32::sin(angle),
            d: 0.0,              e: 1.0, f: 0.0,
            g: -f32::sin(angle), h: 0.0, i: f32::cos(angle),
        }
    }

    #[rustfmt::skip]
    pub fn rotate_z(angle: f32) -> Mat3 {
        Self {
            a: f32::cos(angle), b: -f32::sin(angle), c: 0.0,
            d: f32::sin(angle), e: f32::cos(angle),  f: 0.0,
            g: 0.0,             h: 0.0,              i: 1.0,
        }
    }
}

impl std::ops::Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.a * rhs.x + self.b * rhs.y + self.c * rhs.z,
            self.d * rhs.x + self.e * rhs.y + self.f * rhs.z,
            self.g * rhs.x + self.h * rhs.y + self.i * rhs.z,
        )
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    /// | a, b, c |   | a, b, c |
    /// | d, e, f | x | d, e, f |
    /// | g, h, i |   | g, h, i |
    #[rustfmt::skip]
    fn mul(self, rhs: Mat3) -> Self::Output {
        Mat3::new(
            self.a * rhs.a + self.b * rhs.d + self.c * rhs.g,  self.a * rhs.b + self.b * rhs.e + self.c * rhs.h,  self.a * rhs.c + self.b * rhs.f + self.c * rhs.i,
            self.d * rhs.a + self.e * rhs.d + self.f * rhs.g,  self.d * rhs.b + self.e * rhs.e + self.f * rhs.h,  self.d * rhs.c + self.e * rhs.f + self.f * rhs.i,
            self.g * rhs.a + self.h * rhs.d + self.i * rhs.g,  self.g * rhs.b + self.h * rhs.e + self.i * rhs.h,  self.g * rhs.c + self.h * rhs.f + self.i * rhs.i,
        )
    }
}
