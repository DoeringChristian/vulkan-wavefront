use screen_13::prelude::*;
use std::{collections::HashMap, sync::Arc};

use crate::{buffer::Buffer, dense_arena::Key};

pub struct Accel {
    device: Arc<Device>,
    tlas: Tlas,
    meshes: HashMap<Key, Blas>,
}

impl Accel {
    fn create(device: &Arc<Device>) -> Self {
        Self {
            device: device.clone(),
            tlas: Tlas::create(device, &[]).unwrap(),
            meshes: HashMap::default(),
        }
    }
    fn insert(
        &mut self,
        key: Key,
        indices: &Arc<Buffer<u32>>,
        positions: &Arc<Buffer<glam::Vec3>>,
    ) {
        self.meshes
            .insert(key, Blas::create(&self.device, indices, positions));
    }
    fn update(&self, cache: &mut HashPool, rgraph: &mut RenderGraph) {
        for (key, blas) in self.meshes.iter() {
            blas.build(cache, rgraph);
        }
    }
}

pub struct Blas {
    pub accel: Arc<AccelerationStructure>,
    // Not sure about the use of weaks.
    pub indices: Arc<Buffer<u32>>,
    pub positions: Arc<Buffer<glam::Vec3>>,
    geometry_info: AccelerationStructureGeometryInfo,
    size: AccelerationStructureSize,
}

impl Blas {
    pub fn build(&self, cache: &mut HashPool, rgraph: &mut RenderGraph) {
        //let geometry = scene.geometries.get(self.geometry).unwrap();
        let indices = self.indices.clone();
        let positions = self.positions.clone();
        let index_node = rgraph.bind_node(&indices.buf);
        let vertex_node = rgraph.bind_node(&positions.buf);
        let accel_node = rgraph.bind_node(&self.accel);

        let scratch_buf = rgraph.bind_node(
            cache
                .lease(BufferInfo::new(
                    self.size.build_size,
                    vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
                        | vk::BufferUsageFlags::STORAGE_BUFFER,
                ))
                .unwrap(),
        );

        let triangle_count = indices.count() / 3;
        let geometry_info = self.geometry_info.clone();

        rgraph
            .begin_pass("Build BLAS")
            .read_node(index_node)
            .read_node(vertex_node)
            .write_node(accel_node)
            .write_node(scratch_buf)
            .record_acceleration(move |accel, _| {
                accel.build_structure(
                    accel_node,
                    scratch_buf,
                    &geometry_info,
                    &[vk::AccelerationStructureBuildRangeInfoKHR {
                        first_vertex: 0,
                        primitive_count: triangle_count as u32,
                        primitive_offset: 0,
                        transform_offset: 0,
                    }],
                )
            });
        //AnyAccelerationStructureNode::AccelerationStructure(accel_node)
    }
    // Maybee blas should safe the index of the indices/positions.
    pub fn create(
        device: &Arc<Device>,
        indices: &Arc<Buffer<u32>>,
        positions: &Arc<Buffer<glam::Vec3>>,
    ) -> Self {
        //let triangle_count = geometry.indices.count() / 3;
        let triangle_count = indices.count() / 3;
        let vertex_count = positions.count();
        //let vertex_count = geometry.positions.count();

        let geometry_info = AccelerationStructureGeometryInfo {
            ty: vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
            flags: vk::BuildAccelerationStructureFlagsKHR::empty(),
            geometries: vec![AccelerationStructureGeometry {
                max_primitive_count: triangle_count as _,
                flags: vk::GeometryFlagsKHR::OPAQUE,
                geometry: AccelerationStructureGeometryData::Triangles {
                    index_data: DeviceOrHostAddress::DeviceAddress(
                        screen_13::prelude::Buffer::device_address(&indices.buf),
                    ),
                    index_type: vk::IndexType::UINT32,
                    transform_data: None,
                    max_vertex: vertex_count as _,
                    vertex_data: DeviceOrHostAddress::DeviceAddress(
                        screen_13::prelude::Buffer::device_address(&positions.buf),
                    ),
                    vertex_format: vk::Format::R32G32B32_SFLOAT,
                    vertex_stride: std::mem::size_of::<glam::Vec3>() as _,
                },
            }],
        };

        let accel_size = AccelerationStructure::size_of(device, &geometry_info);

        let accel_info = AccelerationStructureInfo {
            ty: vk::AccelerationStructureTypeKHR::BOTTOM_LEVEL,
            size: accel_size.create_size,
        };

        let accel = AccelerationStructure::create(device, accel_info).unwrap();
        Self {
            //geometry: gkey,
            accel: Arc::new(accel),
            indices: indices.clone(),
            positions: positions.clone(),
            geometry_info,
            size: accel_size,
        }
    }
}

pub struct Tlas {
    instance_buf: Buffer<vk::AccelerationStructureInstanceKHR>,
    pub accel: Arc<AccelerationStructure>,
    //pub instancedata_buf: TypedBuffer<GlslInstanceData>,
    geometry_info: AccelerationStructureGeometryInfo,
    size: AccelerationStructureSize,
}

impl Tlas {
    pub fn build(
        &self,
        //scene: &GpuScene,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
        blas_nodes: &[AnyAccelerationStructureNode],
    ) {
        let scratch_buf = rgraph.bind_node(
            cache
                .lease(BufferInfo::new(
                    self.size.build_size,
                    vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
                        | vk::BufferUsageFlags::STORAGE_BUFFER,
                ))
                .unwrap(),
        );
        let accel_node = rgraph.bind_node(&self.accel);
        let instance_node = rgraph.bind_node(&self.instance_buf.buf);
        let tlas_node = rgraph.bind_node(&self.accel);
        let geometry_info = self.geometry_info.clone();
        //let primitive_count = scene.blases.len();
        let primitive_count = self.instance_buf.count();

        let mut pass = rgraph.begin_pass("Build TLAS");
        for blas_node in blas_nodes {
            //pass = pass.read_node(*blas_node);
            pass = pass.access_node(*blas_node, AccessType::AccelerationStructureBuildRead);
        }
        //pass.read_node(instance_node)
        pass.read_node(instance_node)
            .write_node(scratch_buf)
            .write_node(tlas_node)
            .record_acceleration(move |accel, _| {
                accel.build_structure(
                    accel_node,
                    scratch_buf,
                    &geometry_info,
                    &[vk::AccelerationStructureBuildRangeInfoKHR {
                        primitive_count: primitive_count as _,
                        primitive_offset: 0,
                        first_vertex: 0,
                        transform_offset: 0,
                    }],
                );
            });
        //println!("pass: {:#?}", rgraph);
    }
    pub fn create(
        device: &Arc<Device>,
        //instances_data: &[GlslInstanceData],
        instances: &[vk::AccelerationStructureInstanceKHR],
        //materials: &[GlslMaterial],
    ) -> Option<Self> {
        if (instances.len() == 0) {
            return None;
        }
        // gl_CustomIndexEXT should index into attributes.
        let instance_buf = unsafe {
            Buffer::from_slice_unsafe(
                device,
                &instances,
                vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            )
        };
        let geometry_info = AccelerationStructureGeometryInfo {
            ty: vk::AccelerationStructureTypeKHR::TOP_LEVEL,
            flags: vk::BuildAccelerationStructureFlagsKHR::empty(),
            geometries: vec![AccelerationStructureGeometry {
                max_primitive_count: instances.len() as _,
                flags: vk::GeometryFlagsKHR::OPAQUE,
                geometry: AccelerationStructureGeometryData::Instances {
                    array_of_pointers: false,
                    data: DeviceOrHostAddress::DeviceAddress(
                        screen_13::prelude::Buffer::device_address(&instance_buf.buf),
                    ),
                },
            }],
        };

        let size = AccelerationStructure::size_of(device, &geometry_info);

        let info = AccelerationStructureInfo {
            ty: vk::AccelerationStructureTypeKHR::TOP_LEVEL,
            size: size.create_size,
        };

        let accel = Arc::new(AccelerationStructure::create(device, info).unwrap());

        Some(Self {
            //instancedata_buf,
            instance_buf,
            //material_buf,
            size,
            geometry_info,
            accel,
        })
    }
}
