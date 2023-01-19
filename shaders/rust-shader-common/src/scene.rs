use crate::emitter::Emitter;
use crate::interaction::SurfaceInteraction3f;
use crate::ray::Ray3f;
use spirv_std::glam::*;

pub trait Scene {
    fn ray_intersect(&self, ray: &Ray3f) -> SurfaceInteraction3f;
    fn ray_test(&self, ray: &Ray3f) -> bool;
    fn eval_texture(&self, texture: u32, uv: Vec2) -> Vec3;
    fn emitter(&self, si: &SurfaceInteraction3f) -> Emitter;
}
