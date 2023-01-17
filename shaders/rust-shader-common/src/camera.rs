use spirv_std::glam;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct Camera {
    pub to_world: glam::Mat4,
    pub to_view: glam::Mat4,
    //pub size: glam::UVec2,
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    pub fn perspective(
        to_world: glam::Mat4,
        fov_y: f32,
        aspect_ratio: f32,
        near_clip: f32,
        far_clip: f32,
    ) -> Self {
        Self {
            to_world,
            to_view: glam::Mat4::perspective_rh(fov_y, aspect_ratio, near_clip, far_clip),
            near_clip,
            far_clip,
            //size: glam::uvec2(width, height),
        }
    }
}
