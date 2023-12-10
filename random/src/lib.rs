use vec3::Vec3;

pub struct Random {
    seed: u32,
}

impl Random {
    pub fn new(seed: u32) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(747796405).wrapping_add(2891336453);

        let result = (self
            .seed
            .wrapping_shr(self.seed.wrapping_shr(28).wrapping_add(4))
            ^ self.seed)
            .wrapping_mul(747796405);
        let result = result.wrapping_shr(22) ^ result;

        result as f64 / u32::MAX as f64
    }

    pub fn unit_vec3(&mut self) -> Vec3<f64> {
        loop {
            let p = Vec3::new(
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
            );

            if p.length_squared() < 1.0 {
                let v = p.normalized();

                return v;
            }
        }
    }

    pub fn vec3_on_hemisphere(&mut self, normal: Vec3<f64>) -> Vec3<f64> {
        loop {
            let p = Vec3::new(
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
                self.next() * 2.0 - 1.0,
            );

            if p.length_squared() < 1.0 {
                let v = p.normalized();

                if v.dot(normal) > 0.0 {
                    return v;
                } else {
                    return -v;
                }
            }
        }
    }
}
