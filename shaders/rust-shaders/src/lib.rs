#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use rust_shader_common::*;
use spirv_std::glam;
use spirv_std::ray_tracing::{AccelerationStructure, CandidateIntersection, RayFlags, RayQuery};
use spirv_std::spirv;

#[spirv(compute(threads(64)))]
pub fn ray_intersect(
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
    *hit = HitInfo { t: 1. };
}
