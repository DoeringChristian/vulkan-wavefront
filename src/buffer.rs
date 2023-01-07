use bytemuck::cast_slice;
use crevice::std140::{self, AsStd140};
use screen_13::prelude::{vk, BufferInfo, Device};

use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug)]
pub struct TypedBuffer<T> {
    _ty: PhantomData<T>,
    buf: Arc<screen_13::prelude::Buffer>,
    device: Arc<Device>,
    count: usize,
    stride: usize,
}

impl<T> Deref for TypedBuffer<T> {
    type Target = Arc<screen_13::prelude::Buffer>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T: AsStd140> TypedBuffer<T> {
    pub fn create_from_slice_std140(
        device: &Arc<Device>,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> Self {
        let count = data.len();
        let stride = T::std140_size_static();
        let buf = Arc::new({
            let mut buf = screen_13::prelude::Buffer::create(
                device,
                BufferInfo::new_mappable((stride * count) as u64, usage),
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
            stride,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}
impl<T: bytemuck::Pod> TypedBuffer<T> {
    pub fn create_from_slice(
        device: &Arc<Device>,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> Self {
        let count = data.len();
        let stride = size_of::<T>();
        let buf =
            screen_13::prelude::Buffer::create_from_slice(device, usage, cast_slice(data)).unwrap();
        Self {
            buf: Arc::new(buf),
            count: data.len(),
            stride,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
    pub fn from_slice_mappable(
        device: &Arc<Device>,
        data: &[T],
        usage: vk::BufferUsageFlags,
    ) -> Self {
        let count = data.len();
        let stride = size_of::<T>();
        let mut buf = screen_13::prelude::Buffer::create(
            device,
            BufferInfo::new_mappable((stride * count) as _, usage),
        )
        .unwrap();
        screen_13::prelude::Buffer::copy_from_slice(&mut buf, 0, cast_slice(data));
        Self {
            buf: Arc::new(buf),
            count: data.len(),
            stride,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}
impl<T> TypedBuffer<T> {
    pub unsafe fn from_slice_unsafe(
        device: &Arc<Device>,
        data: &[T],
        usage: vk::BufferUsageFlags,
    ) -> Self {
        let count = data.len();
        let stride = size_of::<T>();
        let data = unsafe {
            std::slice::from_raw_parts(data as *const _ as *const _, data.len() * stride)
        };
        let buf = screen_13::prelude::Buffer::create_from_slice(device, usage, data).unwrap();
        Self {
            buf: Arc::new(buf),
            count: data.len(),
            stride,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}
impl<T> TypedBuffer<T> {
    #[inline]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline]
    pub fn size(&self) -> usize {
        self.count * self.stride
    }
    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }
}
