use core::arch::asm;

pub unsafe fn bitcast_u32_to_f32(a: u32) -> f32 {
    #[cfg(target_arch = "spirv")]
    asm! {
        "%f32 = OpTypeFloat 32",
        "%result = OpBitcast %f32 {a}",
        "OpReturnValue %result",
        a = in(reg) a,
        options(noreturn),
    }
    #[cfg(not(target_arch = "spirv"))]
    0.
}
