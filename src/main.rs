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
    pretty_env_logger::init();
    const SHADER: &[u8] = include_bytes!(env!("rust_shaders.spv"));
    //let device = Arc::new(Device::new(config).unwrap());
    //let arr = Array::create(&device, &[1, 2, 3], BufferUsageFlags::STORAGE_BUFFER);
    let sc13 = EventLoop::new().debug(true).build().unwrap();
    let mut cache = LazyPool::new(&sc13.device);

    let cpplinfo = ComputePipelineInfo::new(SHADER)
        .entry_name("main_cp".into())
        .build();

    let cppl = Arc::new(ComputePipeline::create(&sc13.device, cpplinfo).unwrap());

    let mut rgraph = RenderGraph::new();

    let i = Buffer::from_slice(
        &sc13.device,
        &[0., 1., 2.],
        vk::BufferUsageFlags::STORAGE_BUFFER,
    );
    let o =
        Buffer::from_slice_mappable(&sc13.device, &[0; 3], vk::BufferUsageFlags::STORAGE_BUFFER);

    let i_node = rgraph.bind_node(&i.buf);
    let o_node = rgraph.bind_node(&o.buf);

    rgraph
        .begin_pass("Add 1")
        .bind_pipeline(&cppl)
        .read_descriptor((0, 0), i_node)
        .write_descriptor((0, 1), o_node)
        .record_compute(|compute, _| {
            compute.dispatch(3, 1, 1);
        });

    rgraph.resolve().submit(&mut cache, 0).unwrap();

    let slice: &[f32] = cast_slice(screen_13::prelude::Buffer::mapped_slice(&i));

    println!("{:?}", slice)
}
