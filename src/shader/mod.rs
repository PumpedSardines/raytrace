use metal::*;
use objc::rc::autoreleasepool;
use std::path::PathBuf;

const NUM_SAMPLES: u64 = 2;

fn render() {
    autoreleasepool(|| {
        let device = Device::system_default().expect("No device found");

        let counter_sample_buffer = create_counter_sample_buffer(&device);
        let destination_buffer = device.new_buffer(
            (std::mem::size_of::<u64>() * NUM_SAMPLES as usize) as u64,
            MTLResourceOptions::StorageModeShared,
        );

        let counter_sampling_point = MTLCounterSamplingPoint::AtStageBoundary;
        assert!(device.supports_counter_sampling(counter_sampling_point));

        let command_queue = device.new_command_queue();
        let command_buffer = command_queue.new_command_buffer();

        let compute_pass_descriptor = ComputePassDescriptor::new();
        handle_compute_pass_sample_buffer_attachment(
            compute_pass_descriptor,
            &counter_sample_buffer,
        );
        let encoder =
            command_buffer.compute_command_encoder_with_descriptor(compute_pass_descriptor);

        let pipeline_state = create_pipeline_state(&device);
        encoder.set_compute_pipeline_state(&pipeline_state);

        let (buffer, sum) = create_input_and_output_buffers(&device, num_elements);
        encoder.set_buffer(0, Some(&buffer), 0);
        encoder.set_buffer(1, Some(&sum), 0);

        let num_threads = pipeline_state.thread_execution_width();

        let thread_group_count = MTLSize {
            width: ((num_elements as NSUInteger + num_threads) / num_threads),
            height: 1,
            depth: 1,
        };

        let thread_group_size = MTLSize {
            width: num_threads,
            height: 1,
            depth: 1,
        };

        encoder.dispatch_thread_groups(thread_group_count, thread_group_size);
        encoder.end_encoding();

        resolve_samples_into_buffer(command_buffer, &counter_sample_buffer, &destination_buffer);

        command_buffer.commit();
        command_buffer.wait_until_completed();
        let mut cpu_end = 0;
        let mut gpu_end = 0;
        device.sample_timestamps(&mut cpu_end, &mut gpu_end);

        let ptr = sum.contents() as *mut f32;
        let mut data = vec![];

        unsafe {
            for i in 0..num_elements {
                data.push(*ptr.add(i as usize));
            }
        };

        println!("{:?}", data);
    });
}

fn create_pipeline_state(device: &Device) -> ComputePipelineState {
    let library_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/default.metallib");
    let library = device.new_library_with_file(library_path).unwrap();
    let kernel = library.get_function("sum", None).unwrap();

    let pipeline_state_descriptor = ComputePipelineDescriptor::new();
    pipeline_state_descriptor.set_compute_function(Some(&kernel));

    device
        .new_compute_pipeline_state_with_function(
            pipeline_state_descriptor.compute_function().unwrap(),
        )
        .unwrap()
}

fn handle_compute_pass_sample_buffer_attachment(
    compute_pass_descriptor: &ComputePassDescriptorRef,
    counter_sample_buffer: &CounterSampleBufferRef,
) {
    let sample_buffer_attachment_descriptor = compute_pass_descriptor
        .sample_buffer_attachments()
        .object_at(0)
        .unwrap();

    sample_buffer_attachment_descriptor.set_sample_buffer(counter_sample_buffer);
    sample_buffer_attachment_descriptor.set_start_of_encoder_sample_index(0);
    sample_buffer_attachment_descriptor.set_end_of_encoder_sample_index(1);
}

fn create_input_and_output_buffers(
    device: &Device,
    num_elements: u32,
) -> (metal::Buffer, metal::Buffer) {
    let data = vec![0.0 as f32; num_elements as usize]
        .iter()
        .enumerate()
        .map(|(i, _)| i as f32)
        .map(|i| i as f32)
        .collect::<Vec<_>>();

    let buffer = device.new_buffer_with_data(
        unsafe { std::mem::transmute(data.as_ptr()) },
        (data.len() * std::mem::size_of::<u32>()) as u64,
        MTLResourceOptions::CPUCacheModeDefaultCache,
    );

    let sum = {
        let data = vec![0.0 as f32; num_elements as usize];
        device.new_buffer_with_data(
            unsafe { std::mem::transmute(data.as_ptr()) },
            (data.len() * std::mem::size_of::<u32>()) as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache,
        )
    };
    (buffer, sum)
}
