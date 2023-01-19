use rust_shader_common::*;
use screen_13::prelude::*;
use std::stringify;
use std::sync::Arc;

use crate::array::Array;
use crate::scene::Scene;
use crate::util::cast_slice;

const SHADER: &[u8] = include_bytes!(env!("rust_shaders.spv"));

macro_rules! renderer {
    ($fname:ident => $sname:ident) => {
        pub struct $sname {
            ppl: Arc<ComputePipeline>,
            device: Arc<Device>,
        }

        impl $sname {
            pub fn create(device: &Arc<Device>) -> Self {
                let ppl = Arc::new(
                    ComputePipeline::create(
                        &device,
                        ComputePipelineInfo::default(),
                        Shader::new_compute(SHADER).entry_name(stringify!($fname).into()),
                    )
                    .unwrap(),
                );

                Self {
                    ppl,
                    device: device.clone(),
                }
            }
        }
    };
}

renderer!(path_trace => PTRenderer);

impl PTRenderer {
    pub fn record(
        &self,
        scene: &Scene,
        size: glam::UVec3,
        color: AnyImageNode,
        seed: u32,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
    ) {
        let indices_node = rgraph.bind_node(&scene.indices.buf);
        let positions_node = rgraph.bind_node(&scene.positions.buf);
        let normals_node = rgraph.bind_node(&scene.normals.buf);
        let tangents_node = rgraph.bind_node(&scene.tangents.buf);
        let mesh_data_node = rgraph.bind_node(&scene.mesh_data.as_ref().unwrap().buf);
        let instance_data_node = rgraph.bind_node(&scene.instance_data.as_ref().unwrap().buf);
        let accel_node = rgraph.bind_node(&scene.tlas.as_ref().unwrap().accel);

        let push_constants = PathTracePushConstant {
            sensor: scene.sensors[0],
            seed,
        };

        rgraph
            .begin_pass("IntersectionRenderPass")
            .bind_pipeline(&self.ppl)
            .read_descriptor((0, 0), indices_node)
            .read_descriptor((0, 1), positions_node)
            .read_descriptor((0, 2), normals_node)
            .read_descriptor((0, 3), tangents_node)
            .read_descriptor((0, 4), mesh_data_node)
            .read_descriptor((0, 5), instance_data_node)
            .read_descriptor((0, 6), accel_node)
            .read_descriptor((0, 7), color)
            .record_compute(move |compute, _| {
                compute.push_constants(unsafe { cast_slice(&[push_constants]) });
                compute.dispatch(size.x, size.y, size.z);
            })
            .submit_pass();
    }
}
