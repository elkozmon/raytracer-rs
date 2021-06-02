use crate::vec3::Color;
use std::cmp;
use std::fmt::Display;
use std::fmt::Write;

pub trait ToPPM3 {
    fn to_ppm3(&self) -> u16;
}

impl ToPPM3 for u16 {
    fn to_ppm3(&self) -> u16 {
        *self
    }
}

impl ToPPM3 for f64 {
    fn to_ppm3(&self) -> u16 {
        (self * 256.0) as u16
    }
}

pub struct PPM<T, const WIDTH: usize, const HEIGHT: usize> {
    pixels: Box<[[Color<T>; WIDTH]; HEIGHT]>,
}

impl<T, const WIDTH: usize, const HEIGHT: usize> PPM<T, WIDTH, HEIGHT> {
    pub fn new(pixels: Box<[[Color<T>; WIDTH]; HEIGHT]>) -> Self {
        Self { pixels }
    }
}

impl<T, const WIDTH: usize, const HEIGHT: usize> Display for PPM<T, WIDTH, HEIGHT>
where
    T: ToPPM3,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pixel_str = String::new();
        let mut max_color = 0;

        for pixel_row in self.pixels.iter().rev() {
            for Color { x, y, z } in pixel_row.iter() {
                let r = x.to_ppm3();
                let g = y.to_ppm3();
                let b = z.to_ppm3();

                max_color = cmp::max(max_color, cmp::max(r, cmp::max(g, b)));
                write!(pixel_str, "{} {} {}\n", r, g, b)?;
            }
        }

        write!(f, "P3\n{} {}\n{}\n{}", WIDTH, HEIGHT, max_color, pixel_str)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn display_ppm() {
        let ppm = PPM::<2, 3>::new(Box::new([
            [(1, 10, 20).into(), (5, 2, 5).into()],
            [(5, 7, 4).into(), (1, 5, 4).into()],
            [(1, 10, 21).into(), (5, 2, 5).into()],
        ]));

        assert_eq!(
            "P3\n2 3\n21\n1 10 21\n5 2 5\n5 7 4\n1 5 4\n1 10 20\n5 2 5\n",
            ppm.to_string()
        )
    }
}
