use crevice::std140::AsStd140;
use glam;

#[derive(AsStd140)]
pub struct Interaction3f {
    p: glam::Vec3,
    n: glam::Vec3,
    t: f32,
    time: f32,
}

#[derive(AsStd140)]
pub struct SurfaceInteraction3f {
    interaction: Interaction3f,
    shape_id: u32,
    uv: glam::Vec3,
    wi: glam::Vec3,
}
