use crate::array::Array;
use crate::types::{Ray3, SurfaceInteraction3};
use glam::Vec3;
use screen_13::prelude::*;
use std::sync::Arc;

pub struct Mesh {
    indices: Array<u32>,
    positions: Array<Vec3>,
    device: Arc<Device>,
}

impl Mesh {
    pub fn create(device: &Arc<Device>, indices: &[u32], positions: &[glam::Vec3]) -> Self {
        Self {
            indices: Array::create(device, indices, vk::BufferUsageFlags::STORAGE_BUFFER),
            positions: Array::create(device, positions, vk::BufferUsageFlags::STORAGE_BUFFER),
            device: device.clone(),
        }
    }
    pub fn ray_intersect(&self, ray: Array<Ray3>) -> Array<SurfaceInteraction3> {
        todo!()
    }
}
