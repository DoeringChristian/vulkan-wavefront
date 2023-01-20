use std::f32::consts::PI;
use std::path::Path;

//use buffer::Buffer;
use rust_shader_common::sensor::Sensor;
use screen_13::prelude::*;

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
    let presenter = screen_13_fx::GraphicPresenter::new(&sc13.device).unwrap();

    rgraph.resolve().submit(&mut cache, 0).unwrap();

    unsafe {
        sc13.device.device_wait_idle().unwrap();
    }

    let mut seed = 0;
    sc13.run(|frame| {
        frame.render_graph.clear_color_image(frame.swapchain_image);
        let img = cache
            .lease(ImageInfo::new_2d(
                vk::Format::R32G32B32A32_SFLOAT,
                1000,
                1000,
                vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED,
            ))
            .unwrap();
        let img_node = frame.render_graph.bind_node(img);
        pt_renderer.record(
            &scene,
            glam::uvec3(1000, 1000, 8),
            Some(Sensor::perspective(
                glam::Mat4::from_translation(glam::vec3(0., 0., -300.)),
                PI / 2.,
                1.,
                0.01,
                100.,
            )),
            seed,
            AnyImageNode::ImageLease(img_node),
            &mut cache,
            frame.render_graph,
        );
        presenter.present_image(frame.render_graph, img_node, frame.swapchain_image);
        seed += 1;
    })
    .unwrap();

    // let rays: &[Ray] = unsafe { util::try_cast_slice(Buffer::mapped_slice(&ray)).unwrap() };
    // println!("{:#?}", rays);
}
