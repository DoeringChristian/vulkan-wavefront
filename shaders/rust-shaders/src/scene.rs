use rust_shader_common::{InstanceData, MeshData, Ray};
use spirv_std::glam::*;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};

use crate::interaction::SurfaceInteraction3f;

pub struct Scene<'a> {
    pub indices: &'a [u32],
    pub positions: &'a [u32],
    pub meshes: &'a [MeshData],
    pub instances: &'a [InstanceData],
    pub accel: &'a AccelerationStructure,
}

impl<'a> Scene<'a> {
    pub fn ray_intersect(&self, ray: &Ray) -> SurfaceInteraction3f {
        unsafe {
            spirv_std::ray_query!(let mut query);
            query.initialize(
                self.accel,
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
                let t = query.get_committed_intersection_t();
                SurfaceInteraction3f {
                    t,
                    p: ray.o + ray.d * t,
                    instance_id: query.get_committed_intersection_instance_id(),
                    geometry_idx: query.get_committed_intersection_primitive_index(),
                    valid: 1,
                    ..Default::default()
                }
            } else {
                // ray hit sky
                SurfaceInteraction3f {
                    valid: 0,
                    t: ray.tmax,
                    ..Default::default()
                }
            }
        }
    }
    pub fn ray_test(&self, ray: &Ray) -> bool {
        unsafe {
            spirv_std::ray_query!(let mut query);
            query.initialize(
                self.accel,
                RayFlags::OPAQUE,
                0xff,
                ray.o(),
                ray.tmin,
                ray.d(),
                ray.tmax,
            );
        }
    }
}
