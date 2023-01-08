use crate::accel::{Blas, Tlas};
use crate::buffer::TypedBuffer;
use bytemuck::{Pod, Zeroable};
use russimp::scene::PostProcess;
use russimp::Matrix4x4;
use screen_13::prelude::*;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Range {
    pub start: usize,
    pub end: usize,
}

impl From<std::ops::Range<usize>> for Range {
    fn from(value: std::ops::Range<usize>) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}
impl From<Range> for std::ops::Range<usize> {
    fn from(value: Range) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, Clone)]
struct Mesh {
    indices: std::ops::Range<usize>,
    positions: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct MeshData {
    indices: Range,
    positions: Range,
}

#[derive(Debug, Clone, Copy)]
struct Instance {
    transform: glam::Mat4,
    mesh_idx: usize,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct InstanceData {
    transform: [f32; 16],
    mesh_idx: usize,
}

pub struct Scene {
    device: Arc<Device>,
    positions: Arc<TypedBuffer<glam::Vec3>>,
    indices: Arc<TypedBuffer<u32>>,
    meshes: Vec<Mesh>,
    instances: Vec<Instance>,

    blases: Vec<Blas<glam::Vec3>>,
    tlas: Option<Tlas>,

    instance_data: Option<Arc<TypedBuffer<InstanceData>>>,
    mesh_data: Option<Arc<TypedBuffer<MeshData>>>,
}

fn matrix4x4_to_mat4(src: &Matrix4x4) -> glam::Mat4 {
    glam::Mat4::from_cols_array(&[
        src.a1, src.a2, src.a3, src.a4, src.b1, src.b2, src.b3, src.b4, src.c1, src.c2, src.c3,
        src.c4, src.d1, src.d2, src.d3, src.d4,
    ])
}

fn load_instances(
    instances: &mut Vec<Instance>,
    node: &russimp::node::Node,
    transform: glam::Mat4,
) {
    let transform = transform * matrix4x4_to_mat4(&node.transformation);

    for mesh in node.meshes.iter() {
        instances.push(Instance {
            transform,
            mesh_idx: *mesh as _,
        })
    }

    for child in node.children.iter() {
        load_instances(instances, &child.borrow(), transform);
    }
}

impl Scene {
    pub fn load(device: &Arc<Device>, path: &Path) -> Self {
        let scene = russimp::scene::Scene::from_file(
            path.to_str().unwrap(),
            vec![
                PostProcess::CalculateTangentSpace,
                PostProcess::Triangulate,
                PostProcess::JoinIdenticalVertices,
                PostProcess::SortByPrimitiveType,
            ],
        )
        .unwrap();
        println!("{:#?}", scene);

        let mut positions = vec![];
        let mut indices = vec![];
        let mut meshes = vec![];
        for mesh in scene.meshes.iter() {
            let positions_offset = positions.len();
            for v in mesh.vertices.iter() {
                positions.push(glam::Vec3 {
                    x: v.x,
                    y: v.y,
                    z: v.z,
                })
            }
            let indices_offset = indices.len();
            for face in mesh.faces.iter() {
                indices.push(face.0[0]);
                indices.push(face.0[1]);
                indices.push(face.0[2]);
            }
            meshes.push(Mesh {
                indices: std::ops::Range {
                    start: indices_offset,
                    end: indices.len(),
                },
                positions: std::ops::Range {
                    start: positions_offset,
                    end: positions.len(),
                },
            })
        }
        let mut instances = vec![];

        load_instances(
            &mut instances,
            &scene.root.as_ref().unwrap().borrow(),
            glam::Mat4::IDENTITY,
        );
        Self {
            meshes,
            instances,
            positions: Arc::new(TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                &positions,
            )),
            indices: Arc::new(TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
                &indices,
            )),
            device: device.clone(),

            blases: vec![],
            tlas: None,
            mesh_data: None,
            instance_data: None,
        }
    }
    pub fn upload(&mut self) {
        for instance in self.instances.iter() {
            self.blases.push(Blas::create(
                &self.device,
                &self.indices,
                self.meshes[instance.mesh_idx].indices.clone().into(),
                &self.positions,
            ))
        }
        let instances = self
            .instances
            .iter()
            .enumerate()
            .map(|(i, instance)| vk::AccelerationStructureInstanceKHR {
                transform: vk::TransformMatrixKHR {
                    matrix: [
                        instance.transform.x_axis.x,
                        instance.transform.y_axis.x,
                        instance.transform.z_axis.x,
                        instance.transform.w_axis.x,
                        instance.transform.x_axis.y,
                        instance.transform.y_axis.y,
                        instance.transform.z_axis.y,
                        instance.transform.w_axis.y,
                        instance.transform.x_axis.z,
                        instance.transform.y_axis.z,
                        instance.transform.z_axis.z,
                        instance.transform.w_axis.z,
                    ],
                },
                instance_custom_index_and_mask: vk::Packed24_8::new(i as _, 0xff),
                instance_shader_binding_table_record_offset_and_flags: vk::Packed24_8::new(
                    0,
                    vk::GeometryInstanceFlagsKHR::TRIANGLE_FACING_CULL_DISABLE.as_raw() as _,
                ),
                acceleration_structure_reference: vk::AccelerationStructureReferenceKHR {
                    device_handle: AccelerationStructure::device_address(&self.blases[i].accel),
                },
            })
            .collect::<Vec<_>>();

        self.tlas = Tlas::create(&self.device, &instances);

        let mesh_data = self
            .meshes
            .iter()
            .map(|mesh| MeshData {
                indices: mesh.indices.clone().into(),
                positions: mesh.positions.clone().into(),
            })
            .collect::<Vec<_>>();

        let instance_data = self
            .instances
            .iter()
            .map(|instance| InstanceData {
                transform: instance.transform.to_cols_array(),
                mesh_idx: instance.mesh_idx,
            })
            .collect::<Vec<_>>();

        self.mesh_data = Some(Arc::new(TypedBuffer::create_from_slice(
            &self.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            &mesh_data,
        )));
        self.instance_data = Some(Arc::new(TypedBuffer::create_from_slice(
            &self.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            &instance_data,
        )));
    }
}
