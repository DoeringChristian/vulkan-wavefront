use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder, SpirvMetadata};

fn main() {
    SpirvBuilder::new("rust-shaders", "spirv-unknown-spv1.5")
        //.extension("SPV_KHR_ray_tracing")
        //.extension("SPV_KHR_physical_storage_buffer")
        //.capability(Capability::RayTracingKHR)
        //.capability(Capability::Int64)
        //.capability(Capability::PhysicalStorageBufferAddresses)
        //.capability(Capability::RuntimeDescriptorArray)
        .print_metadata(MetadataPrintout::Full)
        .spirv_metadata(SpirvMetadata::Full)
        .preserve_bindings(true)
        .build()
        .unwrap();
}
