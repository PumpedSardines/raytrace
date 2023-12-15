use crate::{color::Color, gpu::ray_trace, world::World};

use metal::*;

pub struct RayTraceRenderer {
    pipeline_state: ComputePipelineState,
    device: Device,
}

pub struct RayTraceRenderOptions {
    pub image_samples: u32,
    pub pixel_samples: u32,
    pub max_bounces: u32,
}

impl RayTraceRenderOptions {
    pub fn new(image_samples: u32, pixel_samples: u32, max_bounces: u32) -> Self {
        Self {
            image_samples,
            pixel_samples,
            max_bounces,
        }
    }

    pub fn with_image_samples(mut self, image_samples: u32) -> Self {
        self.image_samples = image_samples;
        self
    }

    pub fn with_pixel_samples(mut self, pixel_samples: u32) -> Self {
        self.pixel_samples = pixel_samples;
        self
    }

    pub fn with_max_bounces(mut self, max_bounces: u32) -> Self {
        self.max_bounces = max_bounces;
        self
    }
}

impl Default for RayTraceRenderOptions {
    fn default() -> Self {
        Self {
            image_samples: 1,
            pixel_samples: 10,
            max_bounces: 10,
        }
    }
}

impl RayTraceRenderer {
    pub fn new() -> Self {
        let device = Device::system_default().expect("No device found");
        let pipeline_state = ray_trace::create_pipeline_state(&device);

        Self {
            pipeline_state,
            device,
        }
    }

    pub fn render(&self, world: &World, options: &RayTraceRenderOptions) -> Vec<Color> {
        ray_trace::render(
            &self.device,
            &self.pipeline_state,
            world,
            options.pixel_samples,
            options.image_samples,
            options.max_bounces,
        )
    }
}
