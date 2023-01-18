use bytemuck::cast_slice;
use crevice::std140::{self, AsStd140};
use screen_13::prelude::{vk, BufferInfo, Device};

use crate::util::*;
use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Debug)]
pub struct Array<T> {
    _ty: PhantomData<T>,
    pub buf: Arc<screen_13::prelude::Buffer>,
    device: Arc<Device>,
    count: usize,
    stride: usize,
}

impl<T> Deref for Array<T> {
    type Target = Arc<screen_13::prelude::Buffer>;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

impl<T: Copy> Array<T> {
    pub fn from_slice(device: &Arc<Device>, usage: vk::BufferUsageFlags, data: &[T]) -> Self {
        let count = data.len();
        let stride = size_of::<T>();
        let mut buf = screen_13::prelude::Buffer::create(
            device,
            BufferInfo::new_mappable((count * stride) as _, usage),
        )
        .unwrap();
        screen_13::prelude::Buffer::copy_from_slice(&mut buf, 0, unsafe {
            try_cast_slice(data).unwrap()
        });
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
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> Self {
        let stride = size_of::<T>();
        let buf = screen_13::prelude::Buffer::create_from_slice(
            device,
            usage,
            unsafe { try_cast_slice(data) }.unwrap(),
        )
        .unwrap();
        Self {
            buf: Arc::new(buf),
            count: data.len(),
            stride,
            device: device.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Array<T> {
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
