use std::ops::{Deref, DerefMut};

use bytemuck::{Pod, Zeroable};
use crevice::std140::{AsStd140, Std140};
use glam;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct uint64(pub u64);
impl Deref for uint64 {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for uint64 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl Std140 for uint64 {
    const ALIGNMENT: usize = 64;
}

#[derive(AsStd140)]
pub struct Interaction3 {
    p: glam::Vec3,
    n: glam::Vec3,
    t: f32,
    time: f32,
}

#[derive(AsStd140)]
pub struct SurfaceInteraction3 {
    interaction: Interaction3,
    shape_id: u32,
    uv: glam::Vec3,
    wi: glam::Vec3,
}

#[derive(AsStd140)]
pub struct Ray3 {
    o: glam::Vec3,
    d: glam::Vec3,
}

#[derive(AsStd140)]
pub struct Material {}

#[derive(AsStd140)]
pub struct Instance {
    pub mat: glam::Mat4,

    pub mat_id: u32,
    pub indices: uint64,
    pub vertices: uint64,
}
