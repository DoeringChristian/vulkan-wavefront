use core::f32::consts::PI;

use spirv_std::glam::*;

use crate::interaction::SurfaceInteraction3f;
use crate::scene::Scene;
use crate::texture::Texture;
use crate::warp;

pub struct BSDFSample {
    pub wo: Vec3,
    pub pdf: f32,
}

pub trait BSDF {
    fn sample(
        &self,
        scene: &impl Scene,
        si: &SurfaceInteraction3f,
        sample1: f32,
        sample2: Vec2,
    ) -> (BSDFSample, Vec3);
    fn eval(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> Vec3;
    fn pdf(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> f32;
    fn eval_pdf(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> (Vec3, f32);
    fn eval_pdf_sample(
        &self,
        scene: &impl Scene,
        si: &SurfaceInteraction3f,
        wo: Vec3,
        sample1: f32,
        sample2: Vec2,
    ) -> (Vec3, f32, BSDFSample, Vec3);
}

#[derive(Default, Clone, Copy)]
pub struct DiffuseBsdf {
    pub diffuse: Texture,
}

impl BSDF for DiffuseBsdf {
    fn sample(
        &self,
        scene: &impl Scene,
        si: &SurfaceInteraction3f,
        sample1: f32,
        sample2: Vec2,
    ) -> (BSDFSample, Vec3) {
        let cos_theta_i = si.wi.dot(vec3(0., 0., 1.));

        let wo = warp::square_to_cosine_hemisphere(sample2);
        let pdf = warp::square_to_cosine_hemisphere_pdf(wo);

        let val = self.diffuse.eval(si.uv, scene);

        (BSDFSample { wo, pdf }, val)
    }

    fn eval(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> Vec3 {
        let cos_theta_i = si.wi.dot(vec3(0., 0., 1.));
        let cos_theta_o = wo.dot(vec3(0., 0., 1.));

        if cos_theta_i > 0. && cos_theta_o > 0. {
            self.diffuse.eval(si.uv, scene) * 1. / PI * cos_theta_o
        } else {
            vec3(0., 0., 0.)
        }
    }

    fn pdf(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> f32 {
        let cos_theta_i = si.wi.dot(vec3(0., 0., 1.));
        let cos_theta_o = wo.dot(vec3(0., 0., 1.));

        let pdf = warp::square_to_cosine_hemisphere_pdf(wo);

        if cos_theta_i > 0. && cos_theta_o > 0. {
            pdf
        } else {
            0.
        }
    }

    fn eval_pdf(&self, scene: &impl Scene, si: &SurfaceInteraction3f, wo: Vec3) -> (Vec3, f32) {
        let cos_theta_i = si.wi.dot(vec3(0., 0., 1.));
        let cos_theta_o = wo.dot(vec3(0., 0., 1.));

        let value = self.diffuse.eval(si.uv, scene) * 1. / PI * cos_theta_o;
        let pdf = warp::square_to_cosine_hemisphere_pdf(wo);

        if cos_theta_i > 0. && cos_theta_o > 0. {
            (value, pdf)
        } else {
            (vec3(0., 0., 0.), 0.)
        }
    }

    fn eval_pdf_sample(
        &self,
        scene: &impl Scene,
        si: &SurfaceInteraction3f,
        wo: Vec3,
        sample1: f32,
        sample2: Vec2,
    ) -> (Vec3, f32, BSDFSample, Vec3) {
        todo!()
    }
}
