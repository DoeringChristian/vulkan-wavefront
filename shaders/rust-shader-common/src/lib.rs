#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;

const DEFAULT_TMIN: f32 = 0.001;
const DEFAULT_TMAX: f32 = 10000.0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeshData {
    pub indices: (u32, u32),
    pub positions: (u32, u32),
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstanceData {
    pub transform: glam::Mat4,
    pub mesh_idx: usize,
}

#[derive(Copy, Clone, Default, Debug)]
#[repr(C)]
pub struct HitInfo {
    pub t: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ray {
    pub o: glam::Vec3,
    pub d: glam::Vec3,
    pub tmin: f32,
    pub tmax: f32,
}

impl Ray {
    pub fn new(o: glam::Vec3, d: glam::Vec3) -> Self {
        Self {
            o,
            d: d.normalize(),
            tmin: DEFAULT_TMIN,
            tmax: DEFAULT_TMAX,
        }
    }
    pub fn new_t(o: glam::Vec3, d: glam::Vec3, tmin: f32, tmax: f32) -> Self {
        Self {
            o,
            d: d.normalize(),
            tmin,
            tmax,
        }
    }
}
