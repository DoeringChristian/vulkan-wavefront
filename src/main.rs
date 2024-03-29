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
use crate::renderer::{IntersectionRenderer, RayGenRenderer};
use crate::scene::Scene;

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
    let raygen_renderer = RayGenRenderer::create(&sc13.device);

    let mut cache = HashPool::new(&sc13.device);
    let mut rgraph = RenderGraph::new();

    let mut scene = Scene::load(&sc13.device, &Path::new("assets/scenes/default.fbx"));
    scene.update(&mut cache, &mut rgraph);

    let ray = unsafe {
        Array::from_slice_mappable(
            &sc13.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            &vec![Ray::default(); 1 * 1],
        )
    };

    let sampler = unsafe {
        Array::from_slice_mappable(
            &sc13.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            &vec![IndependentSampler::default(); 1 * 1],
        )
    };
    raygen_renderer.record(
        &ray,
        &sampler,
        scene.cameras[0],
        0,
        1,
        1,
        1,
        &mut cache,
        &mut rgraph,
    );

    // let rays = vec![Ray::new(vec3(0., 0., 0.), vec3(-1., 0., 0.))];
    // let rays = unsafe {
    //     TypedBuffer::unsafe_create_from_slice(
    //         &sc13.device,
    //         vk::BufferUsageFlags::STORAGE_BUFFER,
    //         &rays,
    //     )
    // };
    // let hit_info = unsafe {
    //     TypedBuffer::unsafe_create_mappable_from_slice(
    //         &sc13.device,
    //         vk::BufferUsageFlags::STORAGE_BUFFER,
    //         &vec![HitInfo::default(); 1],
    //     )
    // };
    //
    // intersection_renderer.record(&scene, &rays, &hit_info, &mut cache, &mut rgraph);

    //println!("{:#?}", scene);
    rgraph.resolve().submit(&mut cache, 0).unwrap();

    unsafe {
        sc13.device.device_wait_idle().unwrap();
    }

    let rays: &[Ray] = unsafe { util::try_cast_slice(Buffer::mapped_slice(&ray)).unwrap() };
    println!("{:#?}", rays);
}
