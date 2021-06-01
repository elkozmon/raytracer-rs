mod color;
mod hitable;
mod ppm;
mod ray;
mod sphere;
mod vec3;

use color::Color;
use hitable::Hitable;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    const NX: usize = 800;
    const NY: usize = 400;

    let mut pixels: Box<[[ppm::Pixel; NX]; NY]> = unsafe {
        let layout = std::alloc::Layout::new::<[[ppm::Pixel; NX]; NY]>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut _;
        Box::from_raw(ptr)
    };

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let vec_horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vec_vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let hit: Vec<Sphere<f64>> = vec![
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5),
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0),
    ];

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;

            let ray = Ray::new(origin, lower_left + vec_horizontal * u + vec_vertical * v);

            pixels[j][i] = pixel(ray, &hit);
        }
    }

    let ppm = ppm::PPM::new(pixels);

    std::fs::write("test.ppm", ppm.to_string()).unwrap();
}

fn pixel<H>(ray: Ray<f64>, world: &H) -> ppm::Pixel
where
    H: Hitable<f64>,
{
    if let Some(hit) = world.hit(ray, 0.0, f64::MAX) {
        let n = hit.normal;

        return Color::new(
            (255.99 * n.x) as u16,
            (255.99 * n.y) as u16,
            (255.99 * n.z) as u16,
        );
    };

    let direction = ray.direction.unit();
    let t = 0.5 * (direction.y + 1.0);
    let v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(0.5, 0.7, 1.0);
    let t = v1 * (1.0 - t) + v2 * t;

    Color {
        r: (255.99 * t.x) as u16,
        g: (255.99 * t.y) as u16,
        b: (255.99 * t.z) as u16,
    }
}
