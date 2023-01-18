use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

const DEFAULT_TMIN: f32 = 0.001;
const DEFAULT_TMAX: f32 = 10000.0;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default)]
#[repr(C, align(16))]
pub struct Ray {
    pub o: glam::Vec3,
    pub d: glam::Vec3,
    pub tmin: f32,
    pub tmax: f32,
}
