#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use rust_shader_common::*;
use spirv_std::glam;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};
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
            glam::Vec3::from(ray.o),
            ray.tmin,
            glam::Vec3::from(ray.d),
            ray.tmax,
        );

        while query.proceed() {
            if query.get_candidate_intersection_type() == CandidateIntersection::Triangle {
                query.confirm_intersection();
            } else if query.get_candidate_intersection_type() == CandidateIntersection::AABB {
                query.confirm_intersection();
            }
        }

        if query.get_committed_intersection_type() == CommittedIntersection::Triangle {
            // ray hit triangle
            hit.t = query.get_committed_intersection_t();
            hit.p = (glam::Vec3::from(ray.o) + glam::Vec3::from(ray.d) * hit.t).into();
            hit.instance_id = query.get_committed_intersection_instance_id();
            hit.geometry_index = query.get_committed_intersection_primitive_index();
            hit.valid = 1;
        } else {
            // ray hit sky
            hit.valid = 0;
            hit.t = ray.tmax;
        }
    }
}
