use spirv_std::glam::*;

#[derive(Default)]
pub struct SurfaceInteraction3f {
    pub p: Vec3, // Position of interaction
    pub n: Vec3, // Geometric Normal
    pub barycentric: Vec3,
    pub tbn: Mat3,
    pub t: f32,
    pub instance_id: u32,
    pub geometry_idx: u32,
    pub valid: bool,
}

impl SurfaceInteraction3f {
    pub fn to_local(&self, world_p: Vec3) -> Vec3 {
        self.tbn.inverse() * world_p
    }
    pub fn to_world(&self, local_p: Vec3) -> Vec3 {
        self.tbn * local_p
    }
}
