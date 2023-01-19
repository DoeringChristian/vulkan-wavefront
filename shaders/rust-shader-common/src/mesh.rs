#[derive(Clone)]
#[repr(C)]
pub struct Mesh {
    pub indices: u32,
    pub triangle_count: u32,
    pub positions: u32,
    pub normals: u32,
    pub tangents: u32,
}
