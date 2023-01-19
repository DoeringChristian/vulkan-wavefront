use spirv_std::glam::*;

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Emitter {
    ty: EmitterType,
    instance: u32,
    irradiance: [f32; 3],
}

impl Emitter {
    pub fn env(irradiance: [f32; 3]) -> Self {
        Self {
            ty: EmitterType::Env,
            instance: 0,
            irradiance,
        }
    }
    pub fn area(irradiance: [f32; 3], instance: u32) -> Self {
        Self {
            ty: EmitterType::Area,
            instance,
            irradiance,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum EmitterType {
    Env,
    Area,
}
