use spirv_std::glam::*;

use crate::scene::Scene;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Texture {
    const_val: [f32; 3],
    texture: u32,
    ty: TextureType,
}

impl Texture {
    pub fn constant(val: Vec3) -> Self {
        Self {
            const_val: val.into(),
            texture: 0,
            ty: TextureType::Constant,
        }
    }
    pub fn varying(texture: u32) -> Self {
        Self {
            const_val: [0., 0., 0.],
            texture,
            ty: TextureType::Varying,
        }
    }
    pub fn eval(&self, uv: Vec2, scene: &impl Scene) -> Vec3 {
        match self.ty {
            TextureType::Varying => unimplemented!(),
            TextureType::Constant => Vec3::from_array(self.const_val),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum TextureType {
    Constant,
    Varying,
}
