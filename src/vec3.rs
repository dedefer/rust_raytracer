use std;
use rand;
use std::ops;
use std::fmt;

#[derive(Clone, Copy, Default)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length_squared(&self) -> f64 {
        let Vec3(x, y, z) = self;
        x*x + y*y + z*z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vec3(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.0 * other.0 +
        self.1 * other.1 +
        self.2 * other.2
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn color_str(&self, samples_per_pixel: u64) -> String {
        let scale = 1.0 / samples_per_pixel as f64;
        let Vec3(mut r, mut g, mut b) = *self * scale;
        r = r.sqrt();
        g = g.sqrt();
        b = b.sqrt();

        let clamp = |x: f64| {
            if x < 0.0 { return 0.0 }
            if x > 0.999 { return 0.999 }
            x
        };


        format!(
            "{} {} {}",
            (255.999 * clamp(r)) as u8,
            (255.999 * clamp(g)) as u8,
            (255.999 * clamp(b)) as u8,
        )
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let rand = || { rand::random::<f64>() * (max - min) + min };
        Vec3(rand(), rand(), rand())
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let rnd = || rand::random::<f64>() * 2.0 - 1.0;
        loop {
            let p = Vec3(rnd(), rnd(), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random(-1.0, 1.0).unit_vec3()
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit = Self::random_in_unit_sphere();
        if in_unit.dot(normal) > 0.0 {
            in_unit
        } else {
            -in_unit
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s &&
        self.1.abs() < s &&
        self.2.abs() < s
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        );
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        );
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}


impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
        )
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        self + (-other)
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Vec3 {
        Vec3(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3(
            self.0 * other,
            self.1 * other,
            self.2 * other,
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        other * self
    }

}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        (1.0 / other) * self
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
