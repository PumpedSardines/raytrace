use crate::camera::Camera;
use vec3::{Point3, Vec3};

pub struct CameraBuilder {
    origin: Option<Point3<f64>>,
    direction: Option<Vec3<f64>>,
    focal_length: Option<f64>,
    image_width: Option<u32>,
    image_height: Option<u32>,
    viewport_width: Option<f64>,
}

impl CameraBuilder {
    pub fn new() -> CameraBuilder {
        CameraBuilder {
            origin: None,
            direction: None,
            focal_length: None,
            image_width: None,
            image_height: None,
            viewport_width: None,
        }
    }

    pub fn origin(mut self, origin: Point3<f64>) -> CameraBuilder {
        self.origin = Some(origin);
        self
    }

    pub fn direction(mut self, direction: Vec3<f64>) -> CameraBuilder {
        self.direction = Some(direction);
        self
    }

    pub fn focal_length(mut self, focal_length: f64) -> CameraBuilder {
        self.focal_length = Some(focal_length);
        self
    }

    pub fn image_width(mut self, image_width: u32) -> CameraBuilder {
        self.image_width = Some(image_width);
        self
    }

    pub fn image_height(mut self, image_height: u32) -> CameraBuilder {
        self.image_height = Some(image_height);
        self
    }

    pub fn viewport_width(mut self, viewport_width: f64) -> CameraBuilder {
        self.viewport_width = Some(viewport_width);
        self
    }

    pub fn build(self) -> Camera {
        let direction = self.direction.unwrap();
        let viewport_width = self.viewport_width.unwrap();
        let image_width = self.image_width.unwrap();
        let image_height = self.image_height.unwrap();
        let origin = self.origin.unwrap();
        let focal_length = self.focal_length.unwrap();

        let viewport_u = {
            let x = 1.0;
            let y = 0.0;
            let z = -1.0 * (x * direction.x) / direction.z;

            Vec3::new(x, y, z).normalized().scalar(viewport_width)
        };
        let viewport_v = {
            let viewport_height = (viewport_width / image_width as f64) * image_height as f64;
            viewport_u
                .cross(direction)
                .normalized()
                .scalar(-1.0 * viewport_height)
        };
        let viewport_upper_left = origin - viewport_u.scalar(0.5) - viewport_v.scalar(0.5)
            + direction.normalized().scalar(focal_length);
        let pixel_delta_u = viewport_u.scalar(1.0 / image_width as f64);
        let pixel_delta_v = viewport_v.scalar(1.0 / image_height as f64);

        Camera {
            origin,
            direction,
            image_width,
            image_height,
            viewport_upper_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }
}
