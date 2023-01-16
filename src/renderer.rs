use rust_shader_common::*;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::buffer::TypedBuffer;
use crate::scene::Scene;

const SHADER: &[u8] = include_bytes!(env!("rust_shaders.spv"));

pub struct IntersectionRenderer {
    ppl: Arc<ComputePipeline>,
    device: Arc<Device>,
}
impl IntersectionRenderer {
    pub fn create(device: &Arc<Device>) -> Self {
        let ppl = Arc::new(
            ComputePipeline::create(
                &device,
                ComputePipelineInfo::default(),
                Shader::new_compute(SHADER).entry_name("ray_intersect".into()),
            )
            .unwrap(),
        );

        Self {
            ppl,
            device: device.clone(),
        }
    }

    pub fn record(
        &self,
        scene: &Scene,
        rays: &TypedBuffer<Ray>,
        hit_info: &TypedBuffer<HitInfo>,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
    ) {
        let count = rays.count();
        assert!(count == hit_info.count());

        let indices_node = rgraph.bind_node(&scene.indices.buf);
        let positions_node = rgraph.bind_node(&scene.positions.buf);
        let mesh_data_node = rgraph.bind_node(&scene.mesh_data.as_ref().unwrap().buf);
        let instance_data_node = rgraph.bind_node(&scene.instance_data.as_ref().unwrap().buf);
        let accel_node = rgraph.bind_node(&scene.tlas.as_ref().unwrap().accel);
        let ray_node = rgraph.bind_node(&rays.buf);
        let hit_info_node = rgraph.bind_node(&hit_info.buf);

        rgraph
            .begin_pass("IntersectionRenderPass")
            .bind_pipeline(&self.ppl)
            .read_descriptor((0, 0), indices_node)
            .read_descriptor((0, 1), positions_node)
            .read_descriptor((0, 2), instance_data_node)
            .read_descriptor((0, 4), accel_node)
            .read_descriptor((1, 0), ray_node)
            .write_descriptor((1, 1), hit_info_node)
            .record_compute(move |compute, _| {
                compute.dispatch(count as u32, 1, 1);
            })
            .submit_pass();
    }
}
