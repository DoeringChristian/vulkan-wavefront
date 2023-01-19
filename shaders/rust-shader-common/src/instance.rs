use spirv_std::glam::*;

#[derive(Clone)]
#[repr(C)]
pub struct Instance {
    pub transform: Mat4,
    pub mesh_idx: u32,
    pub emitter: u32,
}
