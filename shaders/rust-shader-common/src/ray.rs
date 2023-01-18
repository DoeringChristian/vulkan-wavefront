use spirv_std::glam::*;

#[derive(Clone, Copy, Default)]
pub struct Ray3f {
    pub o: Vec3,
    pub d: Vec3,
    pub tmin: f32,
    pub tmax: f32,
}
