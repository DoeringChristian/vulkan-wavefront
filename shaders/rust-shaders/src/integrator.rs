use rust_shader_common::Sensor;
use spirv_std::glam::*;

use crate::sampler::IndependentSampler;
use crate::scene::Scene;

pub struct PathIntegrator {}

impl PathIntegrator {
    pub fn render(
        &self,
        scene: &Scene,
        sensor: &Sensor,
        seed: u32,
        idx3: UVec3,
        size: UVec3,
    ) -> Vec4 {
        let idx = idx3.x as usize * size.y as usize * size.z as usize
            + idx3.y as usize * size.z as usize
            + idx3.z as usize;

        let mut sampler = IndependentSampler::new(seed, idx as _);

        let pos = idx3.as_vec3().xy();

        let sample_pos = pos + sampler.next_2d();
        let adjusted_pos = sample_pos / size.as_vec3().xy();

        let ray = sensor.sample_ray(adjusted_pos);

        todo!()
    }
}
