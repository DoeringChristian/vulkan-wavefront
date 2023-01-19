use crate::interaction::SurfaceInteraction3f;
use crate::ray::Ray3f;

pub trait Scene {
    fn ray_intersect(&self, ray: &Ray3f) -> SurfaceInteraction3f;
    fn ray_test(&self, ray: &Ray3f) -> bool;
}
