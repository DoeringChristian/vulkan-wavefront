use crate::buffer::TypedBuffer;
use glam::Vec3;
use screen_13::prelude::*;
use std::sync::Arc;

pub struct Mesh {
    indices: TypedBuffer<u32>,
    positions: TypedBuffer<Vec3>,
    device: Arc<Device>,
}
