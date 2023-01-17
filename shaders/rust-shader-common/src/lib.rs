#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

mod pcg;
mod ray;

pub use pcg::*;
pub use ray::*;

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
pub struct Camera {
    pub to_world: glam::Mat4,
    pub to_view: glam::Mat4,
    pub size: glam::UVec2,
}
