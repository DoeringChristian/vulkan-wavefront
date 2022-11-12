use std::sync::Arc;

use screen_13::prelude::vk::BufferUsageFlags;
use screen_13::prelude::*;

use self::array::Array;
mod array;
mod dense_arena;
mod mesh;
mod scene;
mod types;

fn main() {
    let config = DriverConfig::new().build();
    let device = Arc::new(Device::new(config).unwrap());
    let arr = Array::create(&device, &[1, 2, 3], BufferUsageFlags::STORAGE_BUFFER);
}
