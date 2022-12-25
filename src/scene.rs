use crate::buffer::Buffer;
use glam::Vec3;
use screen_13::prelude::*;
use std::sync::Arc;

pub struct Mesh {
    indices: Buffer<u32>,
    positions: Buffer<Vec3>,
    device: Arc<Device>,
}
