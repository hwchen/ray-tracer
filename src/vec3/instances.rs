use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign
};

use super::Vec3;

/// Color is an instance of Vec3
#[derive(Debug, Clone)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Vec3 for Color {
    fn e0(&self) -> f64 {
        self.r
    }
    fn e1(&self) -> f64 {
        self.g
    }
    fn e2(&self) -> f64 {
        self.b
    }
    fn e0_mut(&mut self) -> &mut f64 {
        &mut self.r
    }
    fn e1_mut(&mut self) -> &mut f64 {
        &mut self.g
    }
    fn e2_mut(&mut self) -> &mut f64 {
        &mut self.b
    }
    fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Color {
            r: e0,
            g: e1,
            b: e2,
        }
    }
}

/// Point is an instance of Vec3
#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 for Point {
    fn e0(&self) -> f64 {
        self.x
    }
    fn e1(&self) -> f64 {
        self.y
    }
    fn e2(&self) -> f64 {
        self.z
    }
    fn e0_mut(&mut self) -> &mut f64 {
        &mut self.x
    }
    fn e1_mut(&mut self) -> &mut f64 {
        &mut self.y
    }
    fn e2_mut(&mut self) -> &mut f64 {
        &mut self.z
    }
    fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Point {
            x: e0,
            y: e1,
            z: e2,
        }
    }
}

/// For all structs that impl Vec3, additional
/// shared traits but that need to be impl through
/// another another trait. This applies mostly
/// to ops. and Display.
/// Since this is for structs that impl Vec3,
/// they all have the constructors and getters that Vec3 requires
/// (e1(), e2(), e3(), new())
macro_rules! ops_impl (
    ($($t:ty),+) => ($(
        impl Add for $t {
            type Output = $t;
            fn add(self, rhs: $t) -> $t {
                Self::new(
                    self.e0() + rhs.e0(),
                    self.e1() + rhs.e1(),
                    self.e2() + rhs.e2(),
                )
            }
        }
        impl Sub for $t {
            type Output = $t;
            fn sub(self, rhs: $t) -> $t {
                Self::new(
                    self.e0() - rhs.e0(),
                    self.e1() - rhs.e1(),
                    self.e2() - rhs.e2(),
                )
            }
        }
        impl Mul for $t {
            type Output = $t;
            fn mul(self, rhs: $t) -> $t {
                Self::new(
                    self.e0() * rhs.e0(),
                    self.e1() * rhs.e1(),
                    self.e2() * rhs.e2(),
                )
            }
        }
        impl Div for $t {
            type Output = $t;
            fn div(self, rhs: $t) -> $t {
                Self::new(
                    self.e0() / rhs.e0(),
                    self.e1() / rhs.e1(),
                    self.e2() / rhs.e2(),
                )
            }
        }
        impl AddAssign for $t {
            fn add_assign(&mut self, rhs: $t) {
                *self.e0_mut() += rhs.e0();
                *self.e1_mut() += rhs.e1();
                *self.e2_mut() += rhs.e2();
            }
        }
        impl SubAssign for $t {
            fn sub_assign(&mut self, rhs: $t) {
                *self.e0_mut() -= rhs.e0();
                *self.e1_mut() -= rhs.e1();
                *self.e2_mut() -= rhs.e2();
            }
        }
        impl MulAssign for $t {
            fn mul_assign(&mut self, rhs: $t) {
                *self.e0_mut() *= rhs.e0();
                *self.e1_mut() *= rhs.e1();
                *self.e2_mut() *= rhs.e2();
            }
        }
        impl DivAssign for $t {
            fn div_assign(&mut self, rhs: $t) {
                *self.e0_mut() /= rhs.e0();
                *self.e1_mut() /= rhs.e1();
                *self.e2_mut() /= rhs.e2();
            }
        }
    )+)
);

macro_rules! scalar_ops_impl (
    ($($t:ty),+; $scalar:ty) => ($(
        impl Add<f64> for $t {
            type Output = $t;
            fn add(self, rhs: $scalar) -> $t {
                Self::new(
                    self.e0() + rhs,
                    self.e1() + rhs,
                    self.e2() + rhs,
                )
            }
        }
        impl Sub<f64> for $t {
            type Output = $t;
            fn sub(self, rhs: $scalar) -> $t {
                Self::new(
                    self.e0() - rhs,
                    self.e1() - rhs,
                    self.e2() - rhs,
                )
            }
        }
        impl Mul<f64> for $t {
            type Output = $t;
            fn mul(self, rhs: $scalar) -> $t {
                Self::new(
                    self.e0() * rhs,
                    self.e1() * rhs,
                    self.e2() * rhs,
                )
            }
        }
        impl Div<f64> for $t {
            type Output = $t;
            fn div(self, rhs: $scalar) -> $t {
                Self::new(
                    self.e0() / rhs,
                    self.e1() / rhs,
                    self.e2() / rhs,
                )
            }
        }
        impl AddAssign<f64> for $t {
            fn add_assign(&mut self, rhs: $scalar) {
                *self.e0_mut() += rhs;
                *self.e1_mut() += rhs;
                *self.e2_mut() += rhs;
            }
        }
        impl SubAssign<f64> for $t {
            fn sub_assign(&mut self, rhs: $scalar) {
                *self.e0_mut() -= rhs;
                *self.e1_mut() -= rhs;
                *self.e2_mut() -= rhs;
            }
        }
        impl MulAssign<f64> for $t {
            fn mul_assign(&mut self, rhs: $scalar) {
                *self.e0_mut() *= rhs;
                *self.e1_mut() *= rhs;
                *self.e2_mut() *= rhs;
            }
        }
        impl DivAssign<f64> for $t {
            fn div_assign(&mut self, rhs: $scalar) {
                *self.e0_mut() /= rhs;
                *self.e1_mut() /= rhs;
                *self.e2_mut() /= rhs;
            }
        }
    )+)
);

ops_impl!(Point, Color);
scalar_ops_impl!(Point, Color; f64);
