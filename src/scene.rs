use crate::buffer::TypedBuffer;
use crate::dense_arena::Arena;
use crate::types::uint64;
use crevice::std140::AsStd140;
use russimp::scene::PostProcess;
use screen_13::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub struct Instance {
    pub transform: glam::Mat4,
}

#[derive(AsStd140)]
pub struct InstanceData {
    pub transform: glam::Mat4,
    pub mesh: uint64,
}

pub struct Mesh {
    pub indices: TypedBuffer<u32>,
    pub positions: TypedBuffer<glam::Vec3>,
    //pub texture_co: Vec<TypedBuffer<glam::Vec2>>,
    pub normals: TypedBuffer<glam::Vec3>,
    pub tangents: TypedBuffer<glam::Vec3>,
}

#[derive(AsStd140)]
pub struct MeshData {
    pub indices: uint64,
    pub positions: uint64,
    //pub texture_co: uint64,
    pub normals: uint64,
    pub tangents: uint64,
}

pub struct Scene {
    meshes: Arena<Mesh>,
    instances: Arena<Instance>,
    device: Arc<Device>,

    mesh_data: Option<TypedBuffer<MeshData>>,
}

impl Scene {
    pub fn load(path: &Path) -> Self {
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
        todo!()
    }
    pub fn update(&mut self) {
        let data = self
            .meshes
            .values()
            .map(|mesh| MeshData {
                indices: uint64(Buffer::device_address(&mesh.indices)),
                positions: uint64(Buffer::device_address(&mesh.positions)),
                normals: uint64(Buffer::device_address(&mesh.normals)),
                tangents: uint64(Buffer::device_address(&mesh.tangents)),
            })
            .collect::<Vec<_>>();
        let data = TypedBuffer::create_from_slice_std140(
            &self.device,
            vk::BufferUsageFlags::STORAGE_BUFFER,
            &data,
        );
        self.mesh_data = Some(data);
    }
}
