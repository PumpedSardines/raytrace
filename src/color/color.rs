#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn grey(value: f64) -> Color {
        Color::new(value, value, value)
    }

    pub fn linear_to_gamma(&self) -> Color {
        Color::new(self.r.sqrt(), self.g.sqrt(), self.b.sqrt())
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        (
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
        )
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Color>>(iter: I) -> Color {
        iter.fold(Color::black(), |a, b| a + b)
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color {
            r: (self.r + rhs.r),
            g: (self.g + rhs.g),
            b: (self.b + rhs.b),
        }
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color {
            r: (self.r - rhs.r),
            g: (self.g - rhs.g),
            b: (self.b - rhs.b),
        }
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self.r * rhs.r),
            g: (self.g * rhs.g),
            b: (self.b * rhs.b),
        }
    }
}
