use std::mem::size_of;

#[inline]
pub unsafe fn try_cast_slice<A, B>(a: &[A]) -> Option<&[B]> {
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
#[inline]
pub unsafe fn cast_slice<A: Copy, B: Copy>(a: &[A]) -> &[B] {
    try_cast_slice(a).unwrap()
}
