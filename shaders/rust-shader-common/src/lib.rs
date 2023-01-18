#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

use core::ops::Range;

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

mod ray;
mod sensor;
//mod path;
//mod pcg;
//mod rand;
//mod ray;
//mod sampler;
//mod warp;

pub use ray::Ray3f;
pub use sensor::Sensor;
//pub use path::PathCtx;
//pub use pcg::PCG;
//pub use rand::sample_tea_32;
//pub use ray::Ray;
//pub use sampler::IndependentSampler;

#[derive(Clone)]
#[repr(C)]
pub struct Mesh {
    pub indices: Range<u32>,
    pub positions: Range<u32>,
    pub normals: Range<u32>,
    pub tangents: Range<u32>,
}

#[derive(Clone)]
#[repr(C)]
pub struct Instance {
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
pub struct PathTracePushConstant {
    pub camera: Sensor,
    pub seed: u32,
    pub spp: u32,
    pub width: u32,
    pub height: u32,
}
