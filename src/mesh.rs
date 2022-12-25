use crate::buffer::Buffer;
use crate::types::{Ray3, SurfaceInteraction3};
use glam::Vec3;
use screen_13::prelude::*;
use std::sync::Arc;

pub struct Mesh {
    indices: Buffer<u32>,
    positions: Buffer<Vec3>,
    device: Arc<Device>,
}

impl Mesh {
    pub fn create(device: &Arc<Device>, indices: &[u32], positions: &[glam::Vec3]) -> Self {
        Self {
            indices: unsafe {
                Buffer::from_slice_unsafe(device, indices, vk::BufferUsageFlags::STORAGE_BUFFER)
            },
            positions: unsafe {
                Buffer::from_slice_unsafe(device, positions, vk::BufferUsageFlags::STORAGE_BUFFER)
            },
            device: device.clone(),
        }
    }
}
