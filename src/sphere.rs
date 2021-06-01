use num_traits::Float;

use crate::{
    hitable::{Hit, Hitable},
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere<T> {
    center: Vec3<T>,
    radius: T,
}

impl<T> Sphere<T> {
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<T> Hitable<T> for Sphere<T>
where
    T: Float + From<f64> + Copy,
{
    fn hit(&self, ray: Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = Into::<T>::into(2.0) * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - Into::<T>::into(4.0) * a * c;

        if !discriminant.is_sign_negative() {
            let t = (-b - (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_time(t);

                let hit = Hit {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                };

                return Some(hit);
            }

            let t = (-b + (b * b - a * c).sqrt()) / a;
            if t < t_max && t > t_min {
                let p = ray.point_at_time(t);

                let hit = Hit {
                    t,
                    p,
                    normal: (p - self.center) / self.radius,
                };

                return Some(hit);
            }
        }

        None
    }
}
