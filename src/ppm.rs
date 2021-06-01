use crate::color::Color;
use std::array;
use std::cmp;
use std::fmt::Display;
use std::fmt::Write;

pub type Pixel = Color<u16>;

pub struct PPM<const WIDTH: usize, const HEIGHT: usize> {
    pixels: Box<[[Pixel; WIDTH]; HEIGHT]>,
}

impl<const WIDTH: usize, const HEIGHT: usize> PPM<WIDTH, HEIGHT> {
    pub fn new(pixels: Box<[[Pixel; WIDTH]; HEIGHT]>) -> Self {
        Self { pixels }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for PPM<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut pixel_str = String::new();
        let mut max_color = 0;

        for pixel_row in self.pixels.iter().rev() {
            for Color { r, g, b } in pixel_row.iter() {
                max_color = cmp::max(max_color, cmp::max(*r, cmp::max(*g, *b)));
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
