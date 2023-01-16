use std::path::Path;
use std::sync::Arc;

//use buffer::Buffer;
use bytemuck::cast_slice;
use screen_13::prelude::vk::BufferUsageFlags;
use screen_13::prelude::*;

use crate::scene::Scene;

//use self::array::Array;
mod accel;
//mod array;
mod buffer;
//mod dense_arena;
//mod sbt;
mod scene;
//mod types;

fn main() {
    pretty_env_logger::init();
    const SHADER: &[u8] = include_bytes!(env!("rust_shaders.spv"));
    //let device = Arc::new(Device::new(config).unwrap());
    //let arr = Array::create(&device, &[1, 2, 3], BufferUsageFlags::STORAGE_BUFFER);
    let sc13 = EventLoop::new()
        .debug(true)
        .ray_tracing(true)
        .build()
        .unwrap();
    let mut cache = HashPool::new(&sc13.device);
    let mut rgraph = RenderGraph::new();

    let mut scene = Scene::load(&sc13.device, &Path::new("assets/scenes/default.fbx"));
    scene.update(&mut cache, &mut rgraph);

    //println!("{:#?}", scene);
    rgraph.resolve().submit(&mut cache, 0).unwrap();

    unsafe {
        sc13.device.device_wait_idle().unwrap();
    }
}
