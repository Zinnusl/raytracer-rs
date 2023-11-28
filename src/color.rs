#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<&[u8]> for Color {
    fn from(pixel: &[u8]) -> Self {
        Color {
            r: pixel[0],
            g: pixel[1],
            b: pixel[2],
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}
