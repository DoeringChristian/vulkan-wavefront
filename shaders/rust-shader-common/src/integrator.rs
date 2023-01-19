use crate::scene::Scene;
use crate::{bsdf::BSDF, ray::Ray3f, sensor::Sensor};
use spirv_std::glam::*;

use crate::sampler::IndependentSampler;

pub struct SimplePathIntegrator {
    pub max_depth: u32,
}

impl SimplePathIntegrator {
    pub fn new() -> Self {
        Self { max_depth: 4 }
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
        let mut depth = 0;
        let mut ray = ray.clone();

        while active && depth < self.max_depth {
            let si = scene.ray_intersect(&ray);
            let bsdf = scene.bsdf(&si);

            let (bsdf_sample, bsdf_weight) =
                bsdf.sample(scene, &si, sampler.next_1d(), sampler.next_2d());

            let bsdf_weight = if bsdf_sample.pdf > 0. {
                bsdf_weight / bsdf_sample.pdf
            } else {
                vec3(0., 0., 0.)
            };

            L += scene.emitter(&si).eval(&si, scene);
            f *= bsdf_weight;

            ray = si.spawn_ray(si.to_world(bsdf_sample.wo));

            depth += 1;

            // DEBUG
            //active = false;
            //L = bsdf_sample.wo;
        }
        return L;
    }
}
