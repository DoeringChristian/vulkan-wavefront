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
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] ray: &[Ray],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] hit: &mut [HitInfo],
) {
    let i = idx.x as usize;
    let hit = &mut hit[i];
    let ray = &ray[i];

    unsafe {
        spirv_std::ray_query!(let mut query);
        query.initialize(
            accel,
            RayFlags::OPAQUE,
            0xff,
            glam::Vec3::from([1., 1., 1.]),
            0.001,
            glam::Vec3::from([-1., -1., -1.]).normalize(),
            10000.,
        );
        if query.proceed() {
            hit.t = 1.;
            if query.get_candidate_intersection_type() == CandidateIntersection::Triangle {
                hit.t = query.get_candidate_intersection_t();
            }
        }
    }
}
