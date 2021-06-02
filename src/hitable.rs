use std::rc::Rc;

use num_traits::Float;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Hit<T> {
    t: T,
    p: Point3<T>,
    normal: Vec3<T>,
    material: Rc<dyn Material<T>>,
    front_face: bool,
}

impl<T> Hit<T>
where
    T: Float + From<f64>,
{
    pub fn new(
        root: T,
        point: Point3<T>,
        outward_normal: Vec3<T>,
        ray: Ray<T>,
        material: Rc<dyn Material<T>>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0.into();

        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            t: root,
            p: point,
            normal,
            material,
            front_face,
        }
    }

    pub fn root(&self) -> T {
        self.t
    }

    pub fn point(&self) -> Point3<T> {
        self.p
    }

    pub fn normal(&self) -> Vec3<T> {
        self.normal
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Rc<dyn Material<T>> {
        self.material.clone()
    }
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
