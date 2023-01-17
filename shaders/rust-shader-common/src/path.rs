use spirv_std::glam;

#[derive(Copy, Clone)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct PathCtx {
    f: f32,
    L: glam::Vec4,
}
