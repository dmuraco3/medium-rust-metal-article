use metal::{objc::rc::autoreleasepool, Buffer, Device, MTLResourceOptions, MTLSize};
use std::{mem::size_of, time::Instant};

const LIB_DATA: &[u8] = include_bytes!("add.metallib");
const SHADER_NAME: &str = "add_vectors";

const LENGTH: u64 = 1_000_000;
const SIZE: u64 = LENGTH * size_of::<f32>() as u64;

fn compute_shader(device: &Device, a: &Buffer, b: &Buffer, c: &mut Buffer) {
    let queue = device.new_command_queue();
    let lib = device.new_library_with_data(LIB_DATA).unwrap();
    let function = lib.get_function(SHADER_NAME, None).unwrap();

    let pipeline = device
        .new_compute_pipeline_state_with_function(&function)
        .unwrap();

    autoreleasepool(|| {
        let command_buffer = queue.new_command_buffer();
        let compute_encoder = command_buffer.new_compute_command_encoder();
        compute_encoder.set_compute_pipeline_state(&pipeline);

        compute_encoder.set_buffer(0, Some(a), 0);
        compute_encoder.set_buffer(1, Some(b), 0);
        compute_encoder.set_buffer(2, Some(c), 0);

        let grid_size = MTLSize::new(LENGTH, 1, 1);
        let threadgroup_size = MTLSize::new(pipeline.thread_execution_width(), 1, 1);

        compute_encoder.dispatch_threads(grid_size, threadgroup_size);

        compute_encoder.end_encoding();
        command_buffer.commit();

        let start = Instant::now();
        command_buffer.wait_until_completed();
        let elapsed = start.elapsed();
        println!("gpu time: {:?}", elapsed);
    })
}

fn main() {
    // Create a new instance of Device
    let device = Device::system_default().expect("No metal device found");

    // Create GPU data buffers in 'Shared' memory
    let mut a_buf = device.new_buffer(SIZE, MTLResourceOptions::StorageModeShared);
    let mut b_buf = device.new_buffer(SIZE, MTLResourceOptions::StorageModeShared);
    let mut c_buf = device.new_buffer(SIZE, MTLResourceOptions::StorageModeShared);

    // Get pointers to data buffers
    let a_ptr: *mut f32 = a_buf.as_mut().contents() as *mut f32;
    let b_ptr: *mut f32 = b_buf.as_mut().contents() as *mut f32;
    let c_ptr: *mut f32 = c_buf.as_mut().contents() as *mut f32;

    // Fill the buffers
    for ii in 0..LENGTH as usize {
        unsafe {
            *a_ptr.add(ii) = 2.23912;
            *b_ptr.add(ii) = 4.78043;
        };
    }

    compute_shader(&device, &a_buf, &b_buf, &mut c_buf);
    
    println!("c[100_000] {:?}", unsafe{*c_ptr.add(900_000)})
}
