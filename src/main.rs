mod color;
mod ppm;
mod ray;
mod vec3;

use color::Color;

use crate::ray::Ray;
use crate::vec3::Vec3;

fn main() {
    const NX: usize = 200;
    const NY: usize = 100;

    let mut pixels: [[ppm::Pixel; NX]; NY] = [[(0, 0, 0).into(); NX]; NY];

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let vec_horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vec_vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..NY).rev() {
        for i in 0..NX {
            let u = i as f64 / NX as f64;
            let v = j as f64 / NY as f64;

            let ray = Ray::new(origin, lower_left + vec_horizontal * u + vec_vertical * v);

            pixels[j][i] = pixel(ray);
        }
    }

    let ppm = ppm::PPM::new(pixels);

    std::fs::write("test.ppm", ppm.to_string()).unwrap();
}

fn ray_hits_sphere(center: Vec3<f64>, radius: f64, ray: Ray<f64>) -> bool {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = 2.0 * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    discriminant > 0.0
}

fn pixel(ray: Ray<f64>) -> ppm::Pixel {
    if ray_hits_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, ray) {
        return Color::new(255, 0, 0);
    }

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
