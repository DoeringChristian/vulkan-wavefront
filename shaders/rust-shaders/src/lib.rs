#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use rust_shader_common::*;
use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};
use spirv_std::spirv;

#[spirv(compute(threads(64)))]
pub fn ray_gen(
    #[spirv(global_invocation_id)] gidx: glam::UVec3,
    #[spirv(push_constant)] push_constant: &RgenPushConstant,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] ray: &mut [Ray],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sampler: &mut [IndependentSampler],
) {
    let camera = push_constant.camera;
    let spp = push_constant.spp;
    let seed = push_constant.seed;

    let idx: usize = gidx.y as usize * camera.size.x as usize * spp as usize
        + gidx.x as usize * spp as usize
        + gidx.z as usize;

    let ray = &mut ray[idx];
    let sampler = &mut sampler[idx];

    sampler.seed(seed, idx as _);

    let sample = sampler.next_2d();
    let pos = glam::vec2(gidx.x as f32, gidx.x as f32); // TODO: maybe use idx to calculate pos
    let adjusted_pos = pos + sample;
    let uv_pos = adjusted_pos / camera.size.as_vec2();

    let view_to_camera = camera.to_view.inverse();

    let near_p = view_to_camera * glam::vec4(uv_pos.x, uv_pos.y, 0., 1.);
    let near_p = near_p.xyz();

    let o = camera.to_world.z_axis.xyz();
    let d = camera.to_world * near_p.normalize().extend(0.);

    *ray = Ray::new(o, d.xyz());
}

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
            ray.o(),
            ray.tmin,
            ray.d(),
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
            hit.p = ray.o + ray.d * hit.t;
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
