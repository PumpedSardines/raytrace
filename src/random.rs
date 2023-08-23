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
}
