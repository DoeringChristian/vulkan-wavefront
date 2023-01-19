use crate::scene::Scene;
use crate::{ray::Ray3f, sensor::Sensor};
use spirv_std::glam::*;

use crate::sampler::IndependentSampler;

pub struct SimplePathIntegrator {}

impl SimplePathIntegrator {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(
        &self,
        scene: &impl Scene,
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

        let L = self.sample(scene, &mut sampler, ray).extend(1.);

        L
    }

    pub fn sample(&self, scene: &impl Scene, sampler: &mut IndependentSampler, ray: Ray3f) -> Vec3 {
        let mut L = vec3(0., 0., 0.);
        let mut f = vec3(1., 1., 1.);
        let mut active = true;

        while active {
            let si = scene.ray_intersect(&ray);

            L = scene.emitter(&si).eval(&si, scene);

            active = false;
        }
        //L = ray.d;
        return L;
    }
}
