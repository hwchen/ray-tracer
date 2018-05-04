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

/// Using a Vec3 trait means that I'll be able to get all of the
/// implementation of Vec3 operations for free once the constructors
/// are implemented.
///
/// At the same time, this allows each type implementing Vec3
/// (e.g. Point, Color) to maintain type safety, so that arbitrary
/// Vec3 cannot be swapped out for another Vec3.
pub trait Vec3 {
    // Constructors that must be defined for each
    // struct that implements Vec3.
    fn e0(&self) -> f64;
    fn e1(&self) -> f64;
    fn e2(&self) -> f64;
    fn e0_mut(&mut self) -> &mut f64;
    fn e1_mut(&mut self) -> &mut f64;
    fn e2_mut(&mut self) -> &mut f64;
    fn new(e0: f64, e1: f64, e2: f64) -> Self;

    // The rest of the methods come for free after a struct
    // implements Vec3.
    fn len(&self) -> f64 {
        let e0 = self.e0();
        let e1 = self.e1();
        let e2 = self.e2();

        (e0*e0 + e1*e1 + e2*e2).sqrt()
    }

    fn square_len(&self) -> f64 {
        let e0 = self.e0();
        let e1 = self.e1();
        let e2 = self.e2();

        (e0*e0 + e1*e1 + e2*e2)
    }

    fn make_unit_vector(&mut self)
        where Self: Sized
    {
        let e0 = self.e0();
        let e1 = self.e1();
        let e2 = self.e2();

        let k = 1. / (e0*e0 + e1*e1 + e2*e2).sqrt();
        *self.e0_mut() = e0*k;
        *self.e1_mut() = e1*k;
        *self.e2_mut() = e2*k;
    }

    fn into_unit_vector(&self) -> Self
        where Self: Sized
    {
        let e0 = self.e0();
        let e1 = self.e1();
        let e2 = self.e2();

        let k = 1. / (e0*e0 + e1*e1 + e2*e2).sqrt();
        Self::new(k*e0, k*e1, k*e2)
    }

    fn to_ppm_tuple(&self) -> String {
        format!("{} {} {}", self.e0(), self.e1(), self.e2())
    }

    fn to_ppm_tuple_int(&self) -> String {
        format!("{} {} {}",
            self.e0() as i64,
            self.e1() as i64,
            self.e2() as i64,
        )
    }
}

/// Point is an instance of Vec3
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
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

/// Color is an instance of Vec3
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
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

pub fn dot<T: Vec3>(v1: &T, v2: &T) -> f64 {
    v1.e0()*v2.e0() +
    v1.e1()*v2.e1() +
    v1.e2()*v2.e2()
}

pub fn cross<T: Vec3>(v1: &T, v2: &T, res: &mut T) {
    *res.e0_mut() = v1.e1() * v2.e2() - v1.e2() * v2.e1();
    *res.e1_mut() = v1.e0() * v2.e2() - v1.e2() * v2.e0();
    *res.e2_mut() = v1.e0() * v2.e1() - v1.e1() * v2.e0();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
