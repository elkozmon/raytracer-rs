use std::ops::{Add, Div, Mul, Neg, Sub};

use num_traits::Float;
use rand::{
    distributions::{
        uniform::{SampleRange, SampleUniform},
        Standard,
    },
    prelude::Distribution,
    thread_rng, Rng,
};

pub type Color<T> = Vec3<T>;
pub type Point3<T> = Vec3<T>;

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vec3<T>
where
    Standard: Distribution<T>,
{
    pub fn random() -> Vec3<T> {
        let mut rng = thread_rng();

        Self::new(rng.gen(), rng.gen(), rng.gen())
    }
}

impl<T> Vec3<T>
where
    T: SampleUniform,
{
    pub fn random_range<R>(range: R) -> Vec3<T>
    where
        R: SampleRange<T> + Clone,
    {
        let mut rng = thread_rng();

        Self::new(
            rng.gen_range(range.clone()),
            rng.gen_range(range.clone()),
            rng.gen_range(range),
        )
    }
}

impl<T> Vec3<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    pub fn dot(&self, rhs: Self) -> T {
        self.x.mul(rhs.x) + self.y.mul(rhs.y) + self.z.mul(rhs.z)
    }
}

impl<T> Vec3<T>
where
    T: Float,
{
    pub fn length(self) -> T {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> T {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn sqrt(self) -> Self {
        Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
    }

    pub fn clamp(self, min: T, max: T) -> Self {
        let f = |t: T| -> T {
            if t.lt(&min) {
                min
            } else if t.gt(&max) {
                max
            } else {
                t
            }
        };

        Self::new(f(self.x), f(self.y), f(self.z))
    }

    pub fn near_zero(self) -> bool {
        const S: f64 = 1e-8;
        self.x.to_f64().unwrap() < S && self.y.to_f64().unwrap() < S && self.z.to_f64().unwrap() < S
    }
}

impl<T> Neg for Vec3<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
            z: self.z.neg(),
        }
    }
}

impl<T> Add for Vec3<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.add(rhs.x),
            y: self.y.add(rhs.y),
            z: self.z.add(rhs.z),
        }
    }
}

impl<T> Sub for Vec3<T>
where
    T: Add<Output = T> + Neg<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl<T> Mul for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.mul(rhs.x),
            y: self.y.mul(rhs.y),
            z: self.z.mul(rhs.z),
        }
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x.mul(rhs),
            y: self.y.mul(rhs),
            z: self.z.mul(rhs),
        }
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x.div(rhs),
            y: self.y.div(rhs),
            z: self.z.div(rhs),
        }
    }
}

impl<T> From<(T, T, T)> for Vec3<T> {
    fn from(t: (T, T, T)) -> Self {
        Self {
            x: t.0,
            y: t.1,
            z: t.2,
        }
    }
}
