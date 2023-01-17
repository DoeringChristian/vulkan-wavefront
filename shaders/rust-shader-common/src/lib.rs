#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

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
}

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct Ray {
    pub o: glam::Vec4,
    pub d: glam::Vec4,
    pub tmin: f32,
    pub tmax: f32,
}

impl Ray {
    pub fn o(&self) -> glam::Vec3 {
        self.o.xyx()
    }
    pub fn d(&self) -> glam::Vec3 {
        self.d.xyz()
    }
    pub fn new(o: glam::Vec3, d: glam::Vec3) -> Self {
        let d = d.normalize();
        Self {
            o: [o.x, o.y, o.z, 0.].into(),
            d: [d.x, d.y, d.z, 0.].into(),
            tmin: DEFAULT_TMIN,
            tmax: DEFAULT_TMAX,
        }
    }
    pub fn new_t(o: glam::Vec3, d: glam::Vec3, tmin: f32, tmax: f32) -> Self {
        let d = d.normalize();
        Self {
            o: [o.x, o.y, o.z, 0.].into(),
            d: [d.x, d.y, d.z, 0.].into(),
            tmin,
            tmax,
        }
    }
}
