use crate::buffer::TypedBuffer;
use crate::dense_arena::Arena;
use russimp::scene::PostProcess;
use screen_13::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub struct Mesh {
    indices: TypedBuffer<u32>,
    positions: TypedBuffer<glam::Vec3>,
    texture_coords: Vec<TypedBuffer<glam::Vec2>>,
    normals: TypedBuffer<glam::Vec3>,
    tangents: TypedBuffer<glam::Vec3>,
}

pub struct Scene {
    meshes: Arena<Mesh>,
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
}
