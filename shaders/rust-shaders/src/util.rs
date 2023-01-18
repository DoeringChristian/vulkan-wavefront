use rust_shader_common::{HitInfo, Ray};
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};

pub fn ray_intersect(ray: &Ray, accel: &AccelerationStructure) -> HitInfo {
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
            let t = query.get_committed_intersection_t();
            HitInfo {
                t,
                p: ray.o + ray.d * t,
                instance_id: query.get_committed_intersection_instance_id(),
                geometry_index: query.get_committed_intersection_primitive_index(),
                valid: 1,
            }
        } else {
            // ray hit sky
            HitInfo {
                valid: 0,
                t: ray.tmax,
                ..Default::default()
            }
        }
    }
}
