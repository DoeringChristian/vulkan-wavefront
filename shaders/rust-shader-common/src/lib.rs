#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

mod camera;
//mod path;
//mod pcg;
//mod rand;
//mod ray;
//mod sampler;
//mod warp;

pub use camera::Camera;
//pub use path::PathCtx;
//pub use pcg::PCG;
//pub use rand::sample_tea_32;
//pub use ray::Ray;
//pub use sampler::IndependentSampler;

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
#[repr(C, align(16))]
pub struct HitInfo {
    pub p: glam::Vec4,
    pub t: f32,
    pub instance_id: u32,
    pub geometry_index: u32,
    pub valid: u32,
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct RgenPushConstant {
    pub camera: Camera,
    pub seed: u32,
    pub spp: u32,
    pub width: u32,
    pub height: u32,
}
