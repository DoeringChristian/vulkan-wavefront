use crate::sensor::Sensor;

#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct PathTracePushConstant {
    pub sensor: Sensor,
    pub seed: u32,
}
