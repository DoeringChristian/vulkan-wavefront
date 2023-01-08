#![no_std]
#![cfg_attr(target_arch = "spirv", feature(asm_experimental_arch,))]

//use bytemuck::{Pod, Zeroable};
use core::arch::asm;
use spirv_std::glam::*;
use spirv_std::spirv;

//pub unsafe fn convert_u_to_ptr<T>(handle: u64) -> *mut T {
//    let result: *mut T;
//    asm!(
//        "{result} = OpConvertUToPtr typeof{result} {handle}",
//        handle = in(reg) handle,
//        result = out(reg) result,
//    );
//    result
//}

#[spirv(compute(threads(64)))]
pub fn main_cp(
    #[spirv(global_invocation_id)] idx: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] i: &[f32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] o: &mut [f32],
) {
    o[idx.x as usize] = i[idx.x as usize] + 1.;
}
