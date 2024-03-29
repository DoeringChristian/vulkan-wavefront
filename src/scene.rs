use crate::accel::{Blas, Tlas};
use crate::array::Array;
use bytemuck::{Pod, Zeroable};
use russimp::scene::PostProcess;
use russimp::Matrix4x4;
use rust_shader_common::*;
use screen_13::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Mesh {
    indices: std::ops::Range<usize>,
    positions: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    transform: glam::Mat4,
    mesh_idx: usize,
}

pub struct Scene {
    device: Arc<Device>,
    pub positions: Arc<Array<glam::Vec3>>,
    pub indices: Arc<Array<u32>>,
    pub meshes: Vec<Mesh>,
    pub instances: Vec<Instance>,
    pub cameras: Vec<Camera>,

    pub blases: Vec<Blas<glam::Vec3>>,
    pub tlas: Option<Tlas>,

    pub instance_data: Option<Arc<Array<InstanceData>>>,
    pub mesh_data: Option<Arc<Array<MeshData>>>,
}

fn matrix4x4_to_mat4(src: &Matrix4x4) -> glam::Mat4 {
    glam::Mat4::from_cols_array(&[
        src.a1, src.b1, src.c1, src.d1, src.a2, src.b2, src.c2, src.d2, src.a3, src.b3, src.c3,
        src.d3, src.a4, src.b4, src.c4, src.d4,
    ])
}

pub struct Node {
    pub node: Rc<RefCell<russimp::node::Node>>,
    pub transform: glam::Mat4,
}

fn load_nodes(
    instances: &mut Vec<Instance>,
    nodes: &mut HashMap<String, Node>,
    node: &Rc<RefCell<russimp::node::Node>>,
    transform: glam::Mat4,
) {
    let transform = transform * matrix4x4_to_mat4(&node.borrow().transformation);

    nodes.insert(
        node.borrow().name.clone(),
        Node {
            node: node.clone(),
            transform,
        },
    );

    for mesh in node.borrow().meshes.iter() {
        instances.push(Instance {
            transform,
            mesh_idx: *mesh as _,
        })
    }

    for child in node.borrow().children.iter() {
        load_nodes(instances, nodes, &child, transform);
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
        let mut nodes = HashMap::new();

        load_nodes(
            &mut instances,
            &mut nodes,
            &scene.root.as_ref().unwrap(),
            glam::Mat4::IDENTITY,
        );

        let cameras = scene
            .cameras
            .iter()
            .map(|camera| {
                let to_world = nodes[&camera.name].transform;
                println!("{:#?}", to_world);
                let fov_x = camera.horizontal_fov;
                let aspect = camera.aspect;
                let fov_y = ((fov_x / 2.).atan() / aspect).tan() * 2.;
                Camera::perspective(
                    to_world,
                    fov_y,
                    aspect,
                    camera.clip_plane_near,
                    camera.clip_plane_far,
                )
            })
            .collect::<Vec<_>>();

        Self {
            meshes,
            instances,
            positions: Arc::new(Array::from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
                    | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR,
                &positions,
            )),
            indices: Arc::new(Array::from_slice(
                device,
                vk::BufferUsageFlags::STORAGE_BUFFER
                    | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS
                    | vk::BufferUsageFlags::ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_KHR,
                &indices,
            )),
            cameras,

            device: device.clone(),

            blases: vec![],
            tlas: None,
            mesh_data: None,
            instance_data: None,
        }
    }
    pub fn update(&mut self, cache: &mut HashPool, rgraph: &mut RenderGraph) {
        // Create blases
        for instance in self.instances.iter() {
            self.blases.push(Blas::create(
                &self.device,
                &self.indices,
                self.meshes[instance.mesh_idx].indices.clone().into(),
                &self.positions,
                self.meshes[instance.mesh_idx].positions.clone().into(),
            ))
        }
        // Transform instances into AccelerationStructureInstanceKHR types
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

        // Create tlas from instances
        self.tlas = Tlas::create(&self.device, &instances);

        // Turn meshs and instances into mesh_data and instance_data
        let mesh_data = self
            .meshes
            .iter()
            .map(|mesh| MeshData {
                indices: (mesh.indices.start as _, mesh.indices.end as _),
                positions: (mesh.positions.start as _, mesh.positions.end as _),
            })
            .collect::<Vec<_>>();

        let instance_data = self
            .instances
            .iter()
            .map(|instance| InstanceData {
                transform: instance.transform,
                mesh_idx: instance.mesh_idx,
            })
            .collect::<Vec<_>>();

        // Upload mesh and instance data
        self.mesh_data = Some(Arc::new(unsafe {
            Array::from_slice_mappable(
                &self.device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &mesh_data,
            )
        }));
        self.instance_data = Some(Arc::new(unsafe {
            Array::from_slice_mappable(
                &self.device,
                vk::BufferUsageFlags::STORAGE_BUFFER,
                &instance_data,
            )
        }));

        // Build blas and tlas
        let blas_nodes = self
            .blases
            .iter()
            .map(|blas| {
                blas.build(cache, rgraph);
                AnyAccelerationStructureNode::AccelerationStructure(rgraph.bind_node(&blas.accel))
            })
            .collect::<Vec<_>>();
        self.tlas
            .as_ref()
            .unwrap()
            .build(cache, rgraph, &blas_nodes);
    }

    pub fn ray_intersect(
        &mut self,
        rays: Array<Ray>,
        cache: &mut HashPool,
        rgraph: &mut RenderGraph,
    ) -> Array<HitInfo> {
        todo!()
    }
}
