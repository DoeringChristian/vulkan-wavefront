use crate::buffer::TypedBuffer;
use bytemuck::{Pod, Zeroable};
use russimp::scene::PostProcess;
use screen_13::prelude::*;
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Slice {
    pub offset: u32,
    pub len: u32,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Mesh {
    indices: Slice,
    positions: Slice,
}

#[derive(Debug, Clone, Copy, Zeroable, Pod)]
#[repr(C)]
struct Instance {
    transform: [f32; 16],
    mesh_idx: u32,
}

#[derive(Debug)]
pub struct Scene {
    device: Arc<Device>,
    positions: TypedBuffer<glam::Vec3>,
    indices: TypedBuffer<u32>,
    meshes: TypedBuffer<Mesh>,
    instances: TypedBuffer<Instance>,
}

fn load_instances(
    instances: &mut Vec<Instance>,
    node: &russimp::node::Node,
    transform: glam::Mat4,
) {
    let node_transform = glam::Mat4::from_cols_array(&[
        node.transformation.a1,
        node.transformation.a2,
        node.transformation.a3,
        node.transformation.a4,
        node.transformation.b1,
        node.transformation.b2,
        node.transformation.b3,
        node.transformation.b4,
        node.transformation.c1,
        node.transformation.c2,
        node.transformation.c3,
        node.transformation.c4,
        node.transformation.d1,
        node.transformation.d2,
        node.transformation.d3,
        node.transformation.d4,
    ]);
    let transform = transform * node_transform;

    for mesh in node.meshes.iter() {
        instances.push(Instance {
            transform: transform.to_cols_array(),
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
        //println!("{:#?}", scene);

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
                indices: Slice {
                    offset: indices_offset as _,
                    len: (indices.len() - indices_offset) as _,
                },
                positions: Slice {
                    offset: positions_offset as _,
                    len: (positions.len() - positions_offset) as _,
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
            meshes: TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &meshes,
            ),
            positions: TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &positions,
            ),
            indices: TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &indices,
            ),
            instances: TypedBuffer::create_from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &instances,
            ),
            device: device.clone(),
        }
    }
    pub fn update(&mut self) {}
}
