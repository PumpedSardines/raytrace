use super::type_mapping;
use crate::color::Color;
use crate::world::World;
use glam::f32::*;
use metal::*;
use objc::rc::autoreleasepool;
use rand::*;

pub(crate) fn render(
    device: &Device,
    pipeline_state: &ComputePipelineState,
    world: &World,
    pixel_samples: u32,
    image_samples: u32,
    max_bounces: u32,
) -> Vec<Color> {
    let width = world.camera.image_width;
    let height = world.camera.image_height;
    let mut rand = rand::thread_rng();

    autoreleasepool(|| {
        let options = RayTraceRenderOptions {
            image_samples,
            pixel_samples,
            max_bounces,
        };
        let buffers = create_buffers(&device, &world, &options);

        let image_samples = 1; //options.image_samples;

        for i in 0..image_samples {
            let command_queue = device.new_command_queue();
            let command_buffer = command_queue.new_command_buffer();

            let compute_pass_descriptor = ComputePassDescriptor::new();
            let encoder =
                command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

            let num_threads = pipeline_state.thread_execution_width();

            let threads_per_thread_group = MTLSize::new(num_threads, num_threads, 1);
            let thread_groups = MTLSize::new(
                (width as u64 + threads_per_thread_group.width - 1)
                    / threads_per_thread_group.width,
                (height as u64 + threads_per_thread_group.height - 1)
                    / threads_per_thread_group.height,
                1,
            );
            encoder.set_compute_pipeline_state(&pipeline_state);

            encoder.set_buffer(0, Some(&buffers.output), 0);
            encoder.set_buffer(1, Some(&buffers.uniforms), 0);
            encoder.set_buffer(2, Some(&buffers.camera), 0);
            encoder.set_buffer(3, Some(&buffers.spheres), 0);
            encoder.set_buffer(4, Some(&buffers.planes), 0);
            encoder.set_buffer(5, Some(&buffers.triangles), 0);

            println!("Pass: {}", i);
            update_uniforms_seed(&buffers.uniforms, rand.gen());

            encoder.dispatch_thread_groups(thread_groups, threads_per_thread_group);
            encoder.end_encoding();
            command_buffer.commit();
            command_buffer.wait_until_completed();
        }
        println!("Done");

        let ptr = buffers.output.contents() as *mut f32;
        let mut data = vec![];

        unsafe {
            for i in 0..width * height {
                let i = i * 3;
                let r = *ptr.add(i as usize) / image_samples as f32;
                let g = *ptr.add(i as usize + 1) / image_samples as f32;
                let b = *ptr.add(i as usize + 2) / image_samples as f32;

                let color = Color::new(r, g, b);

                if i == 0 {
                    println!("{} {} {}", color.r, color.g, color.b);
                }
                data.push(Color::new(color.r, color.g, color.b));
            }
        };

        return data;
    })
}

const SHADER_FILE: &str = super::shader::shader_file();

pub(crate) fn create_pipeline_state(device: &Device) -> ComputePipelineState {
    let library = device
        .new_library_with_source(SHADER_FILE, &CompileOptions::new())
        .unwrap_or_else(|err| {
            println!("Failed to create library: {}", err);
            std::process::exit(1);
        });
    let kernel = library.get_function("ray_trace", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap()
}

pub(crate) struct Buffers {
    camera: metal::Buffer,
    output: metal::Buffer,
    uniforms: metal::Buffer,
    spheres: metal::Buffer,
    planes: metal::Buffer,
    triangles: metal::Buffer,
}

fn update_uniforms_seed(buffer: &metal::Buffer, seed: u32) {
    unsafe {
        let ptr = buffer.contents() as *mut type_mapping::Uniforms;
        (*ptr).seed = seed;
    }
}

struct RayTraceRenderOptions {
    image_samples: u32,
    pixel_samples: u32,
    max_bounces: u32,
}

fn create_buffers(device: &Device, data: &World, options: &RayTraceRenderOptions) -> Buffers {
    // WARNING: THIS FUNCTION IS SUPER UNSAFE
    // I've fucked up multiple times modifying this function.
    // Change it carefully

    let width = data.camera.image_width;
    let height = data.camera.image_height;
    let spheres: &Vec<type_mapping::Sphere> = &data.spheres;
    let planes: &Vec<type_mapping::Plane> = &data.planes;
    let triangles: &Vec<type_mapping::Triangle> = &data.triangles;
    let camera: &type_mapping::Camera = &data.camera;

    let camera = device.new_buffer_with_data(
        unsafe { std::mem::transmute(camera) },
        std::mem::size_of::<type_mapping::Camera>() as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    let uniforms = {
        let uniforms = type_mapping::Uniforms {
            seed: 0,
            samples: options.pixel_samples,
            sphere_count: spheres.len() as u32,
            plane_count: planes.len() as u32,
            triangle_count: triangles.len() as u32,
            max_bounces: options.max_bounces,
        };
        device.new_buffer_with_data(
            unsafe { std::mem::transmute(&uniforms) },
            std::mem::size_of::<type_mapping::Uniforms>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    let spheres = {
        let spheres = match spheres.len() {
            0 => vec![type_mapping::Sphere::default()],
            _ => spheres.to_vec(),
        };

        device.new_buffer_with_data(
            unsafe { std::mem::transmute(spheres.as_ptr()) },
            (spheres.len() * std::mem::size_of::<type_mapping::Sphere>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    let planes = {
        let planes = match planes.len() {
            0 => vec![type_mapping::Plane::default()],
            _ => planes.to_vec(),
        };

        device.new_buffer_with_data(
            unsafe { std::mem::transmute(planes.as_ptr()) },
            (planes.len() * std::mem::size_of::<type_mapping::Plane>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    let triangles = {
        let triangles = match triangles.len() {
            0 => vec![type_mapping::Triangle::default()],
            _ => triangles.to_vec(),
        };

        device.new_buffer_with_data(
            unsafe { std::mem::transmute(triangles.as_ptr()) },
            (triangles.len() * std::mem::size_of::<type_mapping::Triangle>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    let output = {
        let data = vec![0.0; (width * height) as usize * 3];
        device.new_buffer_with_data(
            unsafe { std::mem::transmute(data.as_ptr()) },
            (data.len() * std::mem::size_of::<f32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };

    Buffers {
        camera,
        uniforms,
        triangles,
        spheres,
        planes,
        output,
    }
}
