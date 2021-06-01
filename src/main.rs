mod ppm;

fn main() {
    const NX: usize = 200;
    const NY: usize = 100;

    let mut pixels: [[(u16, u16, u16); NX]; NY] = [[(0, 0, 0); NX]; NY];

    for j in (0..NY).rev() {
        for i in 0..NX {
            let r = i as f64 / NX as f64;
            let g = j as f64 / NY as f64;
            let b = 0.2;
            let ir = (255.99 * r) as u16;
            let ig = (255.99 * g) as u16;
            let ib = (255.99 * b) as u16;

            pixels[j][i] = (ir, ig, ib);
        }
    }

    let ppm = ppm::PPM::new(pixels);

    std::fs::write("test.ppm", ppm.to_string()).unwrap();
}
