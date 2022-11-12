use crate::array::Array;
use glam::Vec3;
use screen_13::prelude::*;
use std::sync::Arc;

pub struct Mesh {
    indices: Array<u32>,
    positions: Array<Vec3>,
    device: Arc<Device>,
}
