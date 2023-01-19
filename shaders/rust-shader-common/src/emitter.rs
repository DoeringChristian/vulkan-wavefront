use spirv_std::glam::*;

#[derive(Clone, Copy)]
pub enum Emitter {
    Area(AreaEmitter),
    Env { irradiance: [f32; 3] },
}

#[derive(Clone, Copy)]
pub struct AreaEmitter {
    pub instance: u32,
}
