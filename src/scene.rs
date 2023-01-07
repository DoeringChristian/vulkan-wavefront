use crate::buffer::TypedBuffer;
use glam::Vec3;
use russimp::scene::PostProcess;
use screen_13::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub struct Mesh {
    indices: TypedBuffer<u32>,
    positions: TypedBuffer<Vec3>,
    device: Arc<Device>,
}

pub struct Scene {}

impl Scene {
    pub fn load(path: &Path) -> Self {
        let scene = russimp::scene::Scene::from_file(
            path.into(),
            vec![
                PostProcess::CalculateTangentSpace,
                PostProcess::Triangulate,
                PostProcess::JoinIdenticalVertices,
                PostProcess::SortByPrimitiveType,
            ],
        )
        .unwrap();
        scene.materials
    }
}
