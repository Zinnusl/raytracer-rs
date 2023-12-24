#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub const fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub fn to_rgb(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
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

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Self {
        Color {
            r: ((self.r as f32 + other.r as f32) / 2.0) as u8,
            g: ((self.g as f32 + other.g as f32) / 2.0) as u8,
            b: ((self.b as f32 + other.b as f32) / 2.0) as u8,
        }
    }
}
