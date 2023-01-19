#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

mod integrator;
mod interaction;
mod rand;
mod sampler;
mod scene;
mod warp;

//use bytemuck::{Pod, Zeroable};
use rust_shader_common::*;
use spirv_std::glam::*;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};
use spirv_std::spirv;
use spirv_std::{glam, Image};

use self::integrator::SimplePathIntegrator;
use self::scene::Scene;

#[spirv(compute(threads(64)))]
pub fn path_trace(
    #[spirv(global_invocation_id)] idx3: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(push_constant)] push_constant: &PathTracePushConstant,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] indices: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] positions: &[Vec3],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] normals: &[Vec3],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] tangents: &[Vec3],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] meshes: &[Mesh],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] instances: &[Instance],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 6)] accel: &AccelerationStructure,
    #[spirv(uniform_constant, descriptor_set = 0, binding = 7)] color: &mut Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
) {
    let idx = idx3.x as usize * size.y as usize * size.z as usize
        + idx3.y as usize * size.z as usize
        + idx3.z as usize;

    let scene = Scene {
        indices,
        positions,
        normals,
        tangents,
        meshes,
        instances,
        accel,
    };

    let integrator = SimplePathIntegrator::new();

    let L = integrator.render(
        &scene,
        &push_constant.sensor,
        push_constant.seed,
        idx3,
        size,
    );

    unsafe { color.write(uvec2(idx3.x, idx3.y), L) };
}
