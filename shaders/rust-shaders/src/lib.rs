#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use core::arch::asm;
use spirv_std::glam;
use spirv_std::ray_tracing::{AccelerationStructure, CandidateIntersection, RayFlags, RayQuery};
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

#[spirv(compute(threads(64)))]
pub fn intersection(
    #[spirv(global_invocation_id)] idx: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] indices: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] positions: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] meshes: &[MeshData],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] instances: &[InstanceData],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 4)] accel: &AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] rays: &[Ray],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] hit: &mut [HitInfo],
) {
    let ray = &rays[idx.x as usize];
    let hit = &mut hit[idx.x as usize];
    *hit = HitInfo::default();
    unsafe {
        spirv_std::ray_query!(let mut query);
        query.initialize(
            accel,
            RayFlags::OPAQUE,
            0xff,
            ray.o,
            ray.tmin,
            ray.d,
            ray.tmax,
        );
        if query.proceed() {
            if query.get_candidate_intersection_type() == CandidateIntersection::Triangle {
                hit.t = query.get_candidate_intersection_t();
            }
        }
    }
}
