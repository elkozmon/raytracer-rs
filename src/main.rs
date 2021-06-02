mod camera;
mod hitable;
mod material;
mod ppm;
mod ray;
mod sphere;
mod vec3;

use std::rc::Rc;

use camera::Camera;
use hitable::Hitable;
use num_traits::ToPrimitive;
use rand::thread_rng;
use rand::Rng;
use ray::Ray;
use sphere::Sphere;
use vec3::Color;
use vec3::Point3;
use vec3::Vec3;

use crate::material::Lambertian;
use crate::material::Metal;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: usize = 480;
const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;
const SAMPLES_PP: usize = 100;
const MAX_DEPTH: usize = 40;

fn main() {
    let mut pixels: Box<[[Color<f64>; IMAGE_WIDTH]; IMAGE_HEIGHT]> = unsafe {
        let layout = std::alloc::Layout::new::<[[Color<f64>; IMAGE_WIDTH]; IMAGE_HEIGHT]>();
        let ptr = std::alloc::alloc_zeroed(layout) as *mut _;
        Box::from_raw(ptr)
    };

    let camera = Camera::new(ASPECT_RATIO);

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let mat_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let mat_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8)));
    let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2)));

    let world: Vec<Sphere<f64>> = vec![
        Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground),
        Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center),
        Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left),
        Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right),
    ];

    let mut rng = thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {}...", j);

        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _ in 0..SAMPLES_PP {
                let u = (i as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rng.gen_range(0.0..1.0)) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.ray(u, v);
                color = color + ray_color(ray, &world, MAX_DEPTH);
            }

            pixels[j][i] = (color / SAMPLES_PP.to_f64().unwrap())
                .sqrt()
                .clamp(0.0, 0.999);
        }
    }

    print!("\nDone!\n");

    let ppm = ppm::PPM::new(pixels);

    std::fs::write("test.ppm", ppm.to_string()).unwrap();
}

fn ray_color<H>(ray: Ray<f64>, world: &H, depth: usize) -> Color<f64>
where
    H: Hitable<f64>,
{
    if depth <= 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        return match hit.material().scatter(ray, hit) {
            None => Color::new(0.0, 0.0, 0.0),
            Some((attenuation, scattered)) => attenuation * ray_color(scattered, world, depth - 1),
        };
    };

    let unit_direction = ray.direction.unit();
    let t = 0.5 * (unit_direction.y + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}
