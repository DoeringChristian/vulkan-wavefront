use spirv_std::glam::*;

use crate::interaction::SurfaceInteraction3f;

pub struct BSDFSample {
    pub wo: Vec3,
    pub pdf: f32,
}

pub trait BSDF {
    fn sample(si: &SurfaceInteraction3f, sample1: f32, sample2: Vec2) -> (BSDFSample, f32);
    fn eval(si: &SurfaceInteraction3f, wo: Vec3) -> Vec3;
    fn pdf(si: &SurfaceInteraction3f, wo: Vec3) -> f32;
    fn eval_pdf(si: &SurfaceInteraction3f, wo: Vec3) -> (Vec3, f32);
    fn eval_pdf_sample(
        si: &SurfaceInteraction3f,
        wo: Vec3,
        sample1: f32,
        sample2: Vec2,
    ) -> (Vec3, f32, BSDFSample, Vec3);
}
