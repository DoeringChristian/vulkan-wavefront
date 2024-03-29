use spirv_std::glam;
use spirv_std::glam::Vec4Swizzles;

const DEFAULT_TMIN: f32 = 0.001;
const DEFAULT_TMAX: f32 = 10000.0;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Copy, Clone, Default)]
#[repr(C, align(16))]
pub struct Ray {
    pub o: glam::Vec4,
    pub d: glam::Vec4,
    pub tmin: f32,
    pub tmax: f32,
}

impl Ray {
    pub fn o(&self) -> glam::Vec3 {
        self.o.xyx()
    }
    pub fn d(&self) -> glam::Vec3 {
        self.d.xyz()
    }
    pub fn new(o: glam::Vec3, d: glam::Vec3) -> Self {
        let d = d.normalize();
        Self {
            o: o.extend(0.),
            d: d.extend(0.),
            tmin: DEFAULT_TMIN,
            tmax: DEFAULT_TMAX,
        }
    }
    pub fn new_t(o: glam::Vec3, d: glam::Vec3, tmin: f32, tmax: f32) -> Self {
        let d = d.normalize();
        Self {
            o: o.extend(0.),
            d: d.extend(0.),
            tmin,
            tmax,
        }
    }
}
