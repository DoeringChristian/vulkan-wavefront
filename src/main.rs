use std::sync::Arc;

//use buffer::Buffer;
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

    let spv = inline_spirv::inline_spirv! {
        r#"
#version 450

layout(set = 0, binding = 0)buffer In{
    float i[];
};
layout(set = 0, binding = 1)buffer Out{
    float o[];
};

void main(){
    o[int(gl_GlobalInvocationID.x)] = o[int(gl_GlobalInvocationID.x)];
}
            "#, comp
    }
    .as_slice();

    let cpplinfo = ComputePipelineInfo::new(spv)
        .entry_name("main".into())
        .build();

    let cppl = Arc::new(ComputePipeline::create(&sc13.device, cpplinfo).unwrap());

    let mut rgraph = RenderGraph::new();

    let i = Arc::new(
        Buffer::create_from_slice(
            &sc13.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            cast_slice(&[0.0f32, 1., 2.]),
        )
        .unwrap(),
    );
    let o = Arc::new(
        Buffer::create_from_slice(
            &sc13.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            cast_slice(&[0.0f32; 3]),
        )
        .unwrap(),
    );

    let i_node = rgraph.bind_node(&i);
    let o_node = rgraph.bind_node(&o);

    rgraph
        .begin_pass("Add 1")
        .bind_pipeline(&cppl)
        .read_descriptor((0, 0), i_node)
        .write_descriptor((0, 1), o_node)
        .record_compute(|compute, _| {
            compute.dispatch(3, 1, 1);
        });

    rgraph.resolve().submit(&mut cache, 0).unwrap();

    let slice: &[f32] = cast_slice(screen_13::prelude::Buffer::mapped_slice(&o));

    println!("{:?}", slice)
}
