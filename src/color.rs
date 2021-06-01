#[derive(Clone, Copy)]
pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T> From<(T, T, T)> for Color<T> {
    fn from(t: (T, T, T)) -> Self {
        Self {
            r: t.0,
            g: t.1,
            b: t.2,
        }
    }
}
