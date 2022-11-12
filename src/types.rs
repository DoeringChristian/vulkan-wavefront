use crevice::std140::AsStd140;
use glam;

#[derive(AsStd140)]
pub struct Interaction3 {
    p: glam::Vec3,
    n: glam::Vec3,
    t: f32,
    time: f32,
}

#[derive(AsStd140)]
pub struct SurfaceInteraction3 {
    interaction: Interaction3,
    shape_id: u32,
    uv: glam::Vec3,
    wi: glam::Vec3,
}

#[derive(AsStd140)]
pub struct Ray3 {
    o: glam::Vec3,
    d: glam::Vec3,
}

#[derive(AsStd140)]
pub struct Material {}
