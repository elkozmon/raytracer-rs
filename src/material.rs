use std::ops::{Add, Mul, Neg};

use num_traits::Float;
use rand::distributions::uniform::SampleUniform;

use crate::{
    hitable::Hit,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};

pub trait Material<T>: Send + Sync {
    fn scatter(&self, ray: Ray<T>, hit: Hit<T>) -> Option<(Color<T>, Ray<T>)>;
}

fn random_point_in_unit_sphere<T>() -> Point3<T>
where
    T: Float + From<f64> + SampleUniform,
{
    loop {
        let min = Into::<T>::into(-1.0);
        let max = Into::<T>::into(1.0);
        let vec = Vec3::random_range(min..max);

        if vec.length_squared() < Into::<T>::into(1.0) {
            return vec;
        }
    }
}
pub struct Lambertian<T> {
    albedo: Color<T>,
}

impl<T> Lambertian<T> {
    pub fn new(albedo: Color<T>) -> Self {
        Self { albedo }
    }
}

impl<T> Material<T> for Lambertian<T>
where
    T: Float + From<f64> + SampleUniform + Send + Sync,
{
    fn scatter(&self, ray: Ray<T>, hit: Hit<T>) -> Option<(Color<T>, Ray<T>)> {
        let mut scatter_direction = hit.normal() + random_point_in_unit_sphere().unit();
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal();
        }

        let scattered = Ray::new(hit.point(), scatter_direction);

        Some((self.albedo, scattered))
    }
}

fn reflect<T>(v: Vec3<T>, n: Vec3<T>) -> Vec3<T>
where
    T: Add<Output = T> + Neg<Output = T> + Mul<Output = T> + From<f64> + Copy,
{
    v - n * Into::<T>::into(2.0) * v.dot(n)
}

pub struct Metal<T> {
    albedo: Color<T>,
}

impl<T> Metal<T> {
    pub fn new(albedo: Color<T>) -> Self {
        Self { albedo }
    }
}

impl<T> Material<T> for Metal<T>
where
    T: Float + From<f64> + Send + Sync + Copy,
{
    fn scatter(&self, ray: Ray<T>, hit: Hit<T>) -> Option<(Color<T>, Ray<T>)> {
        let reflected = reflect(ray.direction.unit(), hit.normal());
        let scattered = Ray::new(hit.point(), reflected);

        if scattered.direction.dot(hit.normal()) > Into::<T>::into(0.0) {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
