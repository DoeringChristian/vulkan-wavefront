use spirv_std::glam::*;

#[derive(Default)]
pub struct SurfaceInteraction3f {
    pub p: Vec3, // Position of interaction
    pub n: Vec3, // Geometric Normal
    pub t: f32,
    pub instance_id: u32,
    pub geometry_idx: u32,
    pub valid: bool,
}

impl SurfaceInteraction3f {
    pub fn to_local(world_p: Vec3) -> Vec3 {
        todo!()
    }
    pub fn to_world(local_p: Vec3) -> Vec3 {
        todo!()
    }
}
