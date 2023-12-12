use crate::color::Color;

type ColorStop = (f64, Color);

#[derive(Clone)]
pub struct Gradient {
    colors: Vec<(ColorStop, ColorStop)>,
}

impl Gradient {
    pub fn new(colors: Vec<(f64, Color)>) -> Self {
        let mut colors = colors.clone();
        colors.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let colors = colors
            .clone()
            .into_iter()
            .zip(colors.into_iter().skip(1))
            .collect::<Vec<(ColorStop, ColorStop)>>();

        Self { colors }
    }

    pub fn lerp(&self, t: f64) -> Color {
        for (color1, color2) in &self.colors {
            let (t1, c1) = color1;
            let (t2, c2) = color2;

            if t >= *t1 && t <= *t2 {
                let t = (t - t1) / (t2 - t1);
                return *c1 * Color::grey(1.0 - t) + *c2 * Color::grey(t);
            }
        }

        Color::black()
    }
}

impl From<Color> for Gradient {
    fn from(color: Color) -> Self {
        Self::new(vec![(0.0, color), (1.0, color)])
    }
}
