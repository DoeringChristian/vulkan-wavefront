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

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct HitInfo {
    pub p: [f32; 3],
    pub t: f32,
    pub instance_id: u32,
    pub geometry_index: u32,
    pub valid: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ray {
    pub o: [f32; 3],
    pub d: [f32; 3],
    pub tmin: f32,
    pub tmax: f32,
}

impl Ray {
    pub fn new(o: glam::Vec3, d: glam::Vec3) -> Self {
        Self {
            o: o.into(),
            d: d.normalize().into(),
            tmin: DEFAULT_TMIN,
            tmax: DEFAULT_TMAX,
        }
    }
    pub fn new_t(o: glam::Vec3, d: glam::Vec3, tmin: f32, tmax: f32) -> Self {
        Self {
            o: o.into(),
            d: d.normalize().into(),
            tmin,
            tmax,
        }
    }
}
