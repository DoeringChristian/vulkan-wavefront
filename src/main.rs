use std::sync::Arc;

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

    let cppl = ComputePipeline::create(
        &device,
        ComputePipelineInfo::new(SHADER)
            .entry_name("main_cp".into())
            .build(),
    );
}
