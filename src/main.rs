use std::sync::Arc;

use buffer::Buffer;
use bytemuck::cast_slice;
use screen_13::prelude::vk::BufferUsageFlags;
use screen_13::prelude::*;

//use self::array::Array;
mod accel;
//mod array;
mod buffer;
mod dense_arena;
mod mesh;
mod sbt;
mod scene;
mod types;

fn main() {
    const SHADER: &[u8] = include_bytes!(env!("rust_shaders.spv"));
    //let device = Arc::new(Device::new(config).unwrap());
    //let arr = Array::create(&device, &[1, 2, 3], BufferUsageFlags::STORAGE_BUFFER);
    let cfg = DriverConfig::new().build();
    let device = Arc::new(Device::new(cfg).unwrap());
    let mut cache = LazyPool::new(&device);

    let cpplinfo = ComputePipelineInfo::new(SHADER)
        .entry_name("main_cpp".into())
        .build();

    let cppl = Arc::new(ComputePipeline::create(&device, cpplinfo).unwrap());
    println!("test");

    let mut rgraph = RenderGraph::new();

    let i = Buffer::from_slice(&device, &[0., 1., 2.], vk::BufferUsageFlags::STORAGE_BUFFER);
    let o = Buffer::from_slice(&device, &[0; 3], vk::BufferUsageFlags::STORAGE_BUFFER);

    let i_node = rgraph.bind_node(&i.buf);
    let o_node = rgraph.bind_node(&o.buf);

    println!("test");

    rgraph
        .begin_pass("Add 1")
        .bind_pipeline(&cppl)
        .read_descriptor((0, 0), i_node)
        .write_descriptor((0, 1), o_node)
        .record_compute(|compute, _| {
            compute.dispatch(3, 1, 1);
        });

    println!("test");

    rgraph.resolve().submit(&device.queue, &mut cache).unwrap();
    println!("test");

    let slice: &[f32] = cast_slice(screen_13::prelude::Buffer::mapped_slice(&i));

    println!("{:?}", slice)
}
