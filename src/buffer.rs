use bytemuck::cast_slice;
use crevice::std140::{self, AsStd140};
use screen_13::prelude::{vk, BufferInfo, Device};

use std::marker::PhantomData;
use std::mem::size_of;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[inline]
pub(crate) unsafe fn try_cast_slice<A: Copy, B: Copy>(a: &[A]) -> Option<&[B]> {
    // Note(Lokathor): everything with `align_of` and `size_of` will optimize away
    // after monomorphization.
    if std::mem::align_of::<B>() > std::mem::align_of::<A>()
        && (a.as_ptr() as usize) % std::mem::align_of::<B>() != 0
    {
        None
    } else if size_of::<B>() == size_of::<A>() {
        Some(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, a.len()) })
    } else if size_of::<A>() == 0 || size_of::<B>() == 0 {
        None
    } else if core::mem::size_of_val(a) % size_of::<B>() == 0 {
        let new_len = core::mem::size_of_val(a) / size_of::<B>();
        Some(unsafe { core::slice::from_raw_parts(a.as_ptr() as *const B, new_len) })
    } else {
        None
    }
}

#[derive(Debug)]
pub struct TypedBuffer<T> {
    _ty: PhantomData<T>,
    pub buf: Arc<screen_13::prelude::Buffer>,
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
impl<T: Copy> TypedBuffer<T> {
    pub unsafe fn unsafe_create_from_slice(
        device: &Arc<Device>,
        usage: vk::BufferUsageFlags,
        data: &[T],
    ) -> Self {
        let count = data.len();
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
