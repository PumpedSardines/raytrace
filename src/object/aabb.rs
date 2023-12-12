use crate::ray::Ray;
use vec3::Vec3;

#[derive(Clone, Debug)]
pub struct AABB {
    min: Vec3<f64>,
    max: Vec3<f64>,
}

impl AABB {
    pub fn new(min: Vec3<f64>, max: Vec3<f64>) -> AABB {
        AABB { min, max }
    }

    pub fn combine(a: &AABB, b: &AABB) -> AABB {
        let min = Vec3::new(
            a.min.x.min(b.min.x),
            a.min.y.min(b.min.y),
            a.min.z.min(b.min.z),
        );
        let max = Vec3::new(
            a.max.x.max(b.max.x),
            a.max.y.max(b.max.y),
            a.max.z.max(b.max.z),
        );

        AABB::new(min, max)
    }

    pub fn get_min(&self) -> &Vec3<f64> {
        &self.min
    }

    pub fn get_max(&self) -> &Vec3<f64> {
        &self.max
    }

    fn axis_from_vec3(v: Vec3<f64>, axis: usize) -> f64 {
        match axis {
            0 => v.x,
            1 => v.y,
            2 => v.z,
            _ => panic!("Invalid axis"),
        }
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        // Andrew Kensler method
        // No idea why this works haha

        let mut t_min = t_min;
        let mut t_max = t_max;

        for i in 0..3 {
            let inv_d = 1.0 / Self::axis_from_vec3(r.direction, i);
            let origin = Self::axis_from_vec3(r.origin, i);
            let min = Self::axis_from_vec3(self.min, i);
            let max = Self::axis_from_vec3(self.max, i);

            let mut t0 = (min - origin) * inv_d;
            let mut t1 = (max - origin) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
