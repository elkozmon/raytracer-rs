use std::ops::{Add, Mul};

use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct Ray<T> {
    pub origin: Point3<T>,
    pub direction: Vec3<T>,
}

impl<T> Ray<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy,
{
    pub fn new(origin: Point3<T>, direction: Vec3<T>) -> Self {
        Self { origin, direction }
    }
    
    pub fn point_at_time(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }
}
