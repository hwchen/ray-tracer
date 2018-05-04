pub mod instances;

pub use self::instances::{Color, Point};

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

