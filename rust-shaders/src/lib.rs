#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use core::arch::asm;
use spirv_std::glam::*;
use spirv_std::ray_tracing::{AccelerationStructure, RayFlags, RayQuery};
use spirv_std::spirv;

//pub unsafe fn convert_u_to_ptr<T>(handle: u64) -> *mut T {
//    let result: *mut T;
//    asm!(
//        "{result} = OpConvertUToPtr typeof{result} {handle}",
//        handle = in(reg) handle,
//        result = out(reg) result,
//    );
//    result
//}

#[repr(C)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

#[repr(C)]
pub struct MeshData {
    indices: Range,
    positions: Range,
}

#[repr(C)]
pub struct InstanceData {
    transform: [f32; 16],
    mesh_idx: usize,
}

#[repr(C)]
pub struct HitInfo {
    t: f32,
}

#[spirv(compute(threads(64)))]
pub fn main_cp(
    #[spirv(global_invocation_id)] idx: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] i: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] o: &mut [f32],
) {
    o[idx.x as usize] = i[idx.x as usize] + 1.;
}

#[spirv(compute(threads(64)))]
pub fn intersection(
    #[spirv(global_invocation_id)] idx: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] indices: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] positions: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] meshes: &[MeshData],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] instances: &[InstanceData],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 4)] accel: &AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] hit: &mut [HitInfo],
) {
    spirv_std::ray_query!(let mut query);
    todo!()
}
