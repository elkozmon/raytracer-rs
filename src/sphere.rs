use std::rc::Rc;

use num_traits::Float;

use crate::{
    hitable::{Hit, Hitable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere<T> {
    center: Vec3<T>,
    radius: T,
    material: Rc<dyn Material<T>>,
}

impl<T> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T, material: Rc<dyn Material<T>>) -> Self {
        Self { center, radius, material }
    }
}

impl<T> Hitable<T> for Sphere<T>
where
    T: Float + From<f64> + Copy,
{
    fn hit(&self, ray: Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant.is_sign_negative() {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.point_at_time(root);
        let outward_normal = (point - self.center) / self.radius;
        let hit = Hit::new(root, point, outward_normal, ray, self.material.clone());

        Some(hit)
    }
}
