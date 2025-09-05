use std::sync::mpsc::channel;

use pollster::FutureExt;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub fn run() -> anyhow::Result<()> {
    let instance = wgpu::Instance::new(&Default::default());
    let adapter = instance.request_adapter(&Default::default()).block_on()?;
    let (device, queue) = adapter.request_device(&Default::default()).block_on()?;

    let shader = device.create_shader_module(wgpu::include_wgsl!("sort.wgsl"));

    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: None,
        module: &shader,
        entry_point: None,
        compilation_options: Default::default(),
        cache: Default::default(),
    });

    let input_data = (0u32..128 * 9).rev().collect::<Vec<_>>();

    let data_buffer = device.create_buffer_init(&BufferInitDescriptor {
        label: Some("data"),
        contents: bytemuck::cast_slice(&input_data),
        usage: wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::STORAGE,
    });

    let temp_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("temp"),
        size: data_buffer.size(),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: data_buffer.as_entire_binding(),
        }],
    });

    let mut encoder = device.create_command_encoder(&Default::default());

    let num_threads = 128;

    {
        let num_dispatches = input_data.len() as u32 / num_threads
            + (input_data.len() as u32 % num_threads > 0) as u32;

        println!("num_dispatches: {num_dispatches}");

        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.dispatch_workgroups(num_dispatches, 1, 1);
    }

    encoder.copy_buffer_to_buffer(&data_buffer, 0, &temp_buffer, 0, data_buffer.size());

    queue.submit([encoder.finish()]);

    {
        let (tx, rx) = channel();
        temp_buffer.map_async(wgpu::MapMode::Read, .., move |result| {
            tx.send(result).unwrap()
        });
        device.poll(wgpu::PollType::Wait)?;
        rx.recv()??;

        let output_data = temp_buffer.get_mapped_range(..);
        let u32_data = bytemuck::cast_slice::<_, u32>(&output_data);

        println!("{:?}", &u32_data[..10]);
        
        // Confirm that the list is sorted
        for i in 1..u32_data.len() {
            assert!(u32_data[i] > u32_data[i - 1], "{}, {}", u32_data[i - 1], u32_data[i]);
        }
    }

    temp_buffer.unmap();

    println!("Success!");

    Ok(())
}