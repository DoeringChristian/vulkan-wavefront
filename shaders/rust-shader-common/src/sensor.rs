use spirv_std::glam::*;

use crate::ray::Ray3f;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct Sensor {
    pub to_world: Mat4,
    pub to_view: Mat4,
    //pub size: UVec2,
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Sensor {
    pub fn perspective(
        to_world: Mat4,
        fov_y: f32,
        aspect_ratio: f32,
        near_clip: f32,
        far_clip: f32,
    ) -> Self {
        let to_view = Mat4::perspective_rh(fov_y, aspect_ratio, near_clip, far_clip);
        let to_view = Mat4::from_translation(vec3(1., 1., 0.)) * to_view;
        let to_view = Mat4::from_scale(vec3(0.5, 0.5, 1.)) * to_view;
        Self {
            to_world,
            to_view,
            near_clip,
            far_clip,
            //size: glam::uvec2(width, height),
        }
    }

    pub fn sample_ray(&self, position_sample: Vec2) -> Ray3f {
        let mut ray = Ray3f::default();

        let view_to_camera = self.to_view.inverse();

        let near_p = view_to_camera * vec4(position_sample.x, position_sample.y, 0., 1.);
        let near_p = near_p.xyz();

        let o = self.to_world.w_axis.xyz();
        let d = near_p.normalize();

        ray.o = o;
        ray.d = (self.to_world * near_p.normalize().extend(0.))
            .normalize()
            .xyz();

        let near_t = self.near_clip / -d.z;
        let far_t = self.far_clip / -d.z;

        //ray.tmin = near_t;
        //ray.tmax = far_t - near_t;
        ray.tmin = 0.001;
        ray.tmax = 10000.;

        ray
    }
}
