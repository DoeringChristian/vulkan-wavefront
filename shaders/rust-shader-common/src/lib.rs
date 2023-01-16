#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use spirv_std::glam;

//pub unsafe fn convert_u_to_ptr<T>(handle: u64) -> *mut T {
//    let result: *mut T;
//    asm!(
//        "{result} = OpConvertUToPtr typeof{result} {handle}",
//        handle = in(reg) handle,
//        result = out(reg) result,
//    );
//    result
//}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MeshData {
    indices: Range,
    positions: Range,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct InstanceData {
    transform: [f32; 16],
    mesh_idx: usize,
}

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct HitInfo {
    t: f32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Ray {
    o: glam::Vec3,
    d: glam::Vec3,
    tmin: f32,
    tmax: f32,
}
