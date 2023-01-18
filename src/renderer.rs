use crate::util;
use rust_shader_common::*;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::array::Array;
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
        rays: &Array<Ray>,
        hit_info: &Array<HitInfo>,
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
            .read_descriptor((0, 2), mesh_data_node)
            .read_descriptor((0, 3), instance_data_node)
            .read_descriptor((0, 4), accel_node)
            .read_descriptor((0, 5), ray_node)
            .write_descriptor((0, 6), hit_info_node)
            .record_compute(move |compute, _| {
                compute.dispatch(count as u32, 1, 1);
            })
            .submit_pass();
    }
}

pub struct RayGenRenderer {
    ppl: Arc<ComputePipeline>,
    device: Arc<Device>,
}
impl RayGenRenderer {
    pub fn create(device: &Arc<Device>) -> Self {
        let ppl = Arc::new(
            ComputePipeline::create(
                &device,
                ComputePipelineInfo::default(),
                Shader::new_compute(SHADER).entry_name("ray_gen".into()),
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
        rays: &Array<Ray>,
        sampler: &Array<IndependentSampler>,
        camera: Camera,
        seed: u32,
        spp: u32,
        width: u32,
        height: u32,
        //spp: u32,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
    ) {
        let count = rays.count();
        assert!(count == sampler.count());
        assert!((width * height * spp) as usize == count);

        let ray_node = rgraph.bind_node(&rays.buf);
        let sampler_node = rgraph.bind_node(&sampler.buf);
        let push_constant = RgenPushConstant {
            camera,
            seed,
            spp,
            width,
            height,
        };

        rgraph
            .begin_pass("IntersectionRenderPass")
            .bind_pipeline(&self.ppl)
            .write_descriptor((0, 0), ray_node)
            .write_descriptor((0, 1), sampler_node)
            .record_compute(move |compute, _| {
                compute.push_constants(unsafe { util::cast_slice(&[push_constant]) });
                compute.dispatch(width, height, spp);
            })
            .submit_pass();
    }
}
