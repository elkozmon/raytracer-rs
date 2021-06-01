use crate::{ray::Ray, vec3::Vec3};

pub struct Hit<T> {
    pub t: T,
    pub p: Vec3<T>,
    pub normal: Vec3<T>,
}

pub trait Hitable<T> {
    fn hit(&self, ray: Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>>;
}

impl<T, U> Hitable<T> for Vec<U>
where
    T: Copy,
    U: Hitable<T>,
{
    fn hit(&self, ray: Ray<T>, t_min: T, t_max: T) -> Option<Hit<T>> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for u in self {
            if let Some(h) = u.hit(ray, t_min, closest_so_far) {
                closest_so_far = h.t;
                hit.replace(h);
            }
        }

        hit
    }
}
