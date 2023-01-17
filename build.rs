use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder, SpirvMetadata};

fn main() {
    SpirvBuilder::new("shaders/rust-shaders", "spirv-unknown-spv1.5")
        .extension("SPV_KHR_ray_query")
        //.extension("SPV_KHR_physical_storage_buffer")
        .capability(Capability::RayQueryKHR)
        .capability(Capability::Int64)
        //.capability(Capability::PhysicalStorageBufferAddresses)
        //.capability(Capability::RuntimeDescriptorArray)
        .print_metadata(MetadataPrintout::Full)
        .spirv_metadata(SpirvMetadata::Full)
        .preserve_bindings(true)
        .build()
        .unwrap();
}
