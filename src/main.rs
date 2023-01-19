use std::f32::consts::PI;
use std::path::Path;
use std::sync::Arc;

//use buffer::Buffer;
use bytemuck::cast_slice;
use glam::f32::vec3;
use glam::uvec2;
use rust_shader_common::*;
use screen_13::prelude::vk::BufferUsageFlags;
use screen_13::prelude::*;

use crate::array::Array;
use crate::scene::Scene;

use self::renderer::PTRenderer;

//use self::array::Array;
mod accel;
//mod array;
mod array;
//mod dense_arena;
//mod sbt;
mod renderer;
mod scene;
mod util;
//mod types;

fn main() {
    pretty_env_logger::init();
    //let device = Arc::new(Device::new(config).unwrap());
    //let arr = Array::create(&device, &[1, 2, 3], BufferUsageFlags::STORAGE_BUFFER);
    let sc13 = EventLoop::new()
        .debug(true)
        .ray_tracing(true)
        .build()
        .unwrap();

    //let intersection_renderer = IntersectionRenderer::create(&sc13.device);
    //let raygen_renderer = RayGenRenderer::create(&sc13.device);

    let mut cache = HashPool::new(&sc13.device);
    let mut rgraph = RenderGraph::new();

    let mut scene = Scene::load(&sc13.device, &Path::new("assets/scenes/default.fbx"));
    scene.update(&mut cache, &mut rgraph);

    let pt_renderer = PTRenderer::create(&sc13.device);

    rgraph.resolve().submit(&mut cache, 0).unwrap();

    unsafe {
        sc13.device.device_wait_idle().unwrap();
    }

    sc13.run(|frame| {
        pt_renderer.record(
            &scene,
            glam::uvec3(100, 100, 8),
            AnyImageNode::SwapchainImage(frame.swapchain_image),
            0,
            &mut cache,
            frame.render_graph,
        );
    })
    .unwrap();

    // let rays: &[Ray] = unsafe { util::try_cast_slice(Buffer::mapped_slice(&ray)).unwrap() };
    // println!("{:#?}", rays);
}
