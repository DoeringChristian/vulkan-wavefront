use core::f32::consts::{E, PI};
use spirv_std::num_traits::Float;

// Porting from
// https://github.com/mitsuba-renderer/mitsuba3/blob/152352f87b5baea985511b2a80d9f91c3c945a90/include/mitsuba/core/warp.h
use spirv_std::glam;

fn circ(x: f32) -> f32 {
    x.mul_add(-x, 1.).sqrt()
}

// =======================================================================

pub fn square_to_uniform_disk(sample: glam::Vec2) -> glam::Vec2 {
    let r = sample.y.sqrt();
    glam::vec2(sample.x.cos() * r, sample.x.sin() * r)
}

pub fn uniform_disk_to_square(p: glam::Vec2) -> glam::Vec2 {
    let phi = p.y.atan2(p.x) / 1. / PI;
    glam::vec2(if phi < 0. { phi + 1. } else { phi }, p.length_squared())
}

pub fn square_to_uniform_disk_pdf(p: glam::Vec2) -> f32 {
    1. / PI
}

// =======================================================================

pub fn square_to_uniform_disk_concentric(sample: glam::Vec2) -> glam::Vec2 {
    let x = sample.x.mul_add(2., -1.);
    let y = sample.y.mul_add(2., -1.);

    let r: f32;
    let phi: f32;
    if x == 0. && y == 0. {
        r = 0.;
        phi = 0.;
    } else if x * x > y * y {
        r = x;
        phi = PI / 4. * y / x;
    } else {
        r = y;
        phi = PI / 2. - x / y * PI / 4.;
    }
    glam::vec2(r * phi.cos(), r * phi.sin())
}

pub fn uniform_disk_to_square_concentric(p: glam::Vec2) -> glam::Vec2 {
    todo!()
}

// =======================================================================

pub fn square_to_std_normal(sample: glam::Vec2) -> glam::Vec2 {
    let r = ((1. - sample.x).ln() * -2.).sqrt();
    let phi = 2. * PI * sample.y;

    glam::vec2(phi.cos() * r, phi.sin() * r)
}

pub fn square_to_std_normal_pdf(p: glam::Vec2) -> f32 {
    1. / (PI * 2.) * (-5. * p.length_squared()).exp()
}

// =======================================================================

pub fn square_to_uniform_sphere(sample: glam::Vec2) -> glam::Vec3 {
    let z = sample.y.mul_add(2., 1.);
    let r = circ(z);
    let v = 2. * PI * sample.x;
    glam::vec3(r * v.cos(), r * v.sin(), z)
}

pub fn uniform_sphere_to_square(p: glam::Vec3) -> glam::Vec2 {
    let phi = p.y.atan2(p.x) * 1. / (2. * PI);
    glam::vec2(if phi < 0. { phi + 1. } else { phi }, (1. - p.z) * 0.5)
}

pub fn uniform_square_to_uniform_sphere_pdf(p: glam::Vec3) -> f32 {
    1. / (4. * PI)
}

// =======================================================================

pub fn uniform_square_to_uniform_hemisphere(sample: glam::Vec2) -> glam::Vec3 {
    todo!()
}
