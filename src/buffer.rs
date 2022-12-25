use bytemuck::cast_slice;
use crevice::std140::{self, AsStd140};
use screen_13::prelude::{vk, BufferInfo, Device};

use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug)]
pub struct Buffer<T> {
    _ty: PhantomData<T>,
    pub buf: Arc<screen_13::prelude::Buffer>,
    device: Arc<Device>,
    count: usize,
    size: usize,
}

impl<T: AsStd140> Buffer<T> {
    pub fn from_slice_std140(
        device: &Arc<Device>,
        data: &[T],
        usage: vk::BufferUsageFlags,
    ) -> Self {
        let count = data.len();
        let size = T::std140_size_static() * count;
        let buf = Arc::new({
            let mut buf = screen_13::prelude::Buffer::create(
                device,
                BufferInfo::new_mappable(size as u64, usage),
            )
            .unwrap();
            let mut writer =
                std140::Writer::new(screen_13::prelude::Buffer::mapped_slice_mut(&mut buf));
            writer.write(data).unwrap();
            buf
        });
        Self {
            buf,
            count: data.len(),
            size,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}
impl<T: bytemuck::Pod> Buffer<T> {
    pub fn from_slice(device: &Arc<Device>, data: &[T], usage: vk::BufferUsageFlags) -> Self {
        let count = data.len();
        let size = size_of::<T>() * count;
        let buf =
            screen_13::prelude::Buffer::create_from_slice(device, usage, cast_slice(data)).unwrap();
        Self {
            buf: Arc::new(buf),
            count: data.len(),
            size,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}
impl<T> Buffer<T> {
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn size(&self) -> usize {
        self.size
    }
}

impl<T> Deref for Buffer<T> {
    type Target = screen_13::prelude::Buffer;

    fn deref(&self) -> &Self::Target {
        self.buf.deref()
    }
}
