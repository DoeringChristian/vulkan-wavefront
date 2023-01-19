use rust_shader_common::instance::Instance;
use rust_shader_common::mesh::Mesh;
use rust_shader_common::ray::Ray3f;
use rust_shader_common::scene::Scene;
use spirv_std::glam::*;
use spirv_std::ray_tracing::{
    AccelerationStructure, CandidateIntersection, CommittedIntersection, RayFlags, RayQuery,
};

use crate::interaction::SurfaceInteraction3f;

pub struct GPUScene<'a> {
    pub indices: &'a [u32],
    pub positions: &'a [[f32; 3]],
    pub normals: &'a [[f32; 3]],
    pub tangents: &'a [[f32; 3]],
    pub meshes: &'a [Mesh],
    pub instances: &'a [Instance],
    pub accel: &'a AccelerationStructure,
}

impl<'a> Scene for GPUScene<'a> {
    fn ray_intersect(&self, ray: &Ray3f) -> SurfaceInteraction3f {
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
                let primitive_idx = query.get_committed_intersection_primitive_index();

                let instance = &self.instances[instance_id as usize];
                let mesh = &self.meshes[instance.mesh_idx as usize];

                //// As slices are not supported yet. We need to index manually
                let triangle = uvec3(
                    self.indices[mesh.indices as usize + primitive_idx as usize * 3 + 0],
                    self.indices[mesh.indices as usize + primitive_idx as usize * 3 + 1],
                    self.indices[mesh.indices as usize + primitive_idx as usize * 3 + 2],
                );

                let p1 =
                    Vec3::from_array(self.positions[mesh.positions as usize + triangle.x as usize]);
                let p2 =
                    Vec3::from_array(self.positions[mesh.positions as usize + triangle.y as usize]);
                let p3 =
                    Vec3::from_array(self.positions[mesh.positions as usize + triangle.z as usize]);

                let p1 = instance.transform.transform_point3(p1);
                let p2 = instance.transform.transform_point3(p2);
                let p3 = instance.transform.transform_point3(p3);

                let normal = (p2 - p1).cross(p3 - p1).normalize();

                let tangent =
                    Vec3::from_array(self.tangents[mesh.tangents as usize + triangle.x as usize])
                        * barycentric.x
                        + Vec3::from_array(
                            self.tangents[mesh.tangents as usize + triangle.y as usize],
                        ) * barycentric.y
                        + Vec3::from_array(
                            self.tangents[mesh.tangents as usize + triangle.z as usize],
                        ) * barycentric.z;

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
                    n: tbn.z_axis,
                    tbn,
                    barycentric,
                    instance_id,
                    geometry_idx: primitive_idx,
                    wi: -ray.d,
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
    fn ray_test(&self, ray: &Ray3f) -> bool {
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
