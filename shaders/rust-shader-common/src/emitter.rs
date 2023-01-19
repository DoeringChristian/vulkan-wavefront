use spirv_std::glam::*;

use crate::interaction::SurfaceInteraction3f;
use crate::scene::Scene;
use crate::texture::Texture;

#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Emitter {
    ty: EmitterType,
    instance: u32,
    irradiance: Texture,
}

impl Emitter {
    pub fn env(irradiance: Texture) -> Self {
        Self {
            ty: EmitterType::Env,
            instance: 0,
            irradiance,
        }
    }
    pub fn area(irradiance: Texture, instance: u32) -> Self {
        Self {
            ty: EmitterType::Area,
            instance,
            irradiance,
        }
    }
    pub fn none() -> Self {
        Self {
            ty: EmitterType::None,
            instance: 0,
            irradiance: Texture::constant(vec3(0., 0., 0.)),
        }
    }

    pub fn eval(&self, si: &SurfaceInteraction3f, scene: &impl Scene) -> Vec3 {
        self.irradiance.eval(si.uv, scene)
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum EmitterType {
    None,
    Env,
    Area,
}
