use spirv_std::glam::*;

#[derive(Clone, Copy)]
pub enum Emitter {
    AreaEmitter(AreaEmitter),
}

#[derive(Clone, Copy)]
pub struct AreaEmitter {
    pub mesh: u32,
}
