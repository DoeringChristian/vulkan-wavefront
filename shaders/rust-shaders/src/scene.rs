use rust_shader_common::{Instance, Mesh, Ray3f};
use spirv_std::glam::*;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};

use crate::interaction::SurfaceInteraction3f;

pub struct Scene<'a> {
    pub indices: &'a [u32],
    pub positions: &'a [Vec3],
    pub normals: &'a [Vec3],
    pub tangents: &'a [Vec3],
    pub meshes: &'a [Mesh],
    pub instances: &'a [Instance],
    pub accel: &'a AccelerationStructure,
}

impl<'a> Scene<'a> {
    pub fn ray_intersect(&self, ray: &Ray3f) -> SurfaceInteraction3f {
        unsafe {
            spirv_std::ray_query!(let mut query);
            query.initialize(
                self.accel,
                RayFlags::OPAQUE,
                0xff,
                ray.o,
                ray.tmin,
                ray.d,
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
                let barycentric: Vec2 = query.get_committed_intersection_barycentrics();
                let barycentric = vec3(
                    1. - barycentric.x - barycentric.y,
                    barycentric.x,
                    barycentric.y,
                );

                let t = query.get_committed_intersection_t();

                let instance_id = query.get_committed_intersection_instance_id();
                let geometry_idx = query.get_committed_intersection_primitive_index();

                let instance = &self.instances[instance_id as usize];
                let mesh = &self.meshes[instance.mesh_idx as usize];

                //let indices = &self.indices[mesh.indices.start as usize..mesh.indices.end as usize];
                //let positions = &self.positions[mesh.positions.start as _..mesh.positions.end as _];
                //let normals = &self.normals[mesh.normals.start as _..mesh.normals.end as _];
                //let tangents = &self.tangents[mesh.tangents.start as _..mesh.normals.end as _];

                // As slices are not supported yet I need to index manually
                let triangle = uvec3(
                    self.indices[mesh.indices.start as usize + geometry_idx as usize * 3 + 0],
                    self.indices[mesh.indices.start as usize + geometry_idx as usize * 3 + 1],
                    self.indices[mesh.indices.start as usize + geometry_idx as usize * 3 + 2],
                );

                let normal = self.normals[mesh.normals.start as usize + triangle.x as usize]
                    * barycentric.x
                    + self.normals[mesh.normals.start as usize + triangle.y as usize]
                        * barycentric.y
                    + self.normals[mesh.normals.start as usize + triangle.z as usize]
                        * barycentric.z;

                let tangent = self.tangents[mesh.tangents.start as usize + triangle.x as usize]
                    * barycentric.x
                    + self.tangents[mesh.tangents.start as usize + triangle.y as usize]
                        * barycentric.y
                    + self.tangents[mesh.tangents.start as usize + triangle.z as usize]
                        * barycentric.z;

                let bitangent = normal.cross(tangent);

                let tbn = mat3(tangent, bitangent, normal);
                // Normal Transform usin inverse transpose
                let normal_matrix = instance.transform.inverse().transpose();
                let normal_matrix = mat3(
                    normal_matrix.x_axis.xyz(),
                    normal_matrix.y_axis.xyz(),
                    normal_matrix.z_axis.xyz(),
                );
                let tbn = normal_matrix * tbn;

                SurfaceInteraction3f {
                    t,
                    p: ray.o + ray.d * t,
                    //n: normal,
                    //tbn,
                    barycentric,
                    instance_id: query.get_committed_intersection_instance_id(),
                    geometry_idx: query.get_committed_intersection_primitive_index(),
                    valid: true,
                    ..Default::default()
                }
            } else {
                // ray hit sky
                SurfaceInteraction3f {
                    valid: false,
                    t: ray.tmax,
                    ..Default::default()
                }
            }
        }
    }
    pub fn ray_test(&self, ray: &Ray3f) -> bool {
        unsafe {
            spirv_std::ray_query!(let mut query);
            query.initialize(
                self.accel,
                RayFlags::OPAQUE,
                0xff,
                ray.o,
                ray.tmin,
                ray.d,
                ray.tmax,
            );
            query.proceed()
        }
    }
}
