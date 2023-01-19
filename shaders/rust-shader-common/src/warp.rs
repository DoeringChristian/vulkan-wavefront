use core::f32::consts::{E, PI};
use spirv_std::num_traits::Float;

// Porting from
// https://github.com/mitsuba-renderer/mitsuba3/blob/152352f87b5baea985511b2a80d9f91c3c945a90/include/mitsuba/core/warp.h
use spirv_std::glam::*;

fn circ(x: f32) -> f32 {
    x.mul_add(-x, 1.).sqrt()
}

// =======================================================================

pub fn square_to_uniform_disk(sample: Vec2) -> Vec2 {
    let r = sample.y.sqrt();
    vec2(sample.x.cos() * r, sample.x.sin() * r)
}

pub fn uniform_disk_to_square(p: Vec2) -> Vec2 {
    let phi = p.y.atan2(p.x) / 1. / PI;
    vec2(if phi < 0. { phi + 1. } else { phi }, p.length_squared())
}

pub fn square_to_uniform_disk_pdf(p: Vec2) -> f32 {
    1. / PI
}

// =======================================================================

pub fn square_to_uniform_disk_concentric(sample: Vec2) -> Vec2 {
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
    vec2(r * phi.cos(), r * phi.sin())
}

pub fn uniform_disk_to_square_concentric(p: Vec2) -> Vec2 {
    let quadrant_0_or_2 = p.x.abs() > p.y.abs();
    let r_sign = if quadrant_0_or_2 { p.x } else { p.y };
    let r = p.length().copysign(r_sign);

    let phi = (p.y * r_sign.signum()).atan2(p.x * r_sign.signum());

    let t = 4. / PI * phi;
    let t = if quadrant_0_or_2 { t } else { 2. - t } * r;

    let a = if quadrant_0_or_2 { r } else { t };
    let b = if quadrant_0_or_2 { t } else { r };

    vec2((a + 1.) * 0.5, (b + 1.) * 0.5)
}

// =======================================================================

pub fn square_to_std_normal(sample: Vec2) -> Vec2 {
    let r = ((1. - sample.x).ln() * -2.).sqrt();
    let phi = 2. * PI * sample.y;

    vec2(phi.cos() * r, phi.sin() * r)
}

pub fn square_to_std_normal_pdf(p: Vec2) -> f32 {
    1. / (PI * 2.) * (-5. * p.length_squared()).exp()
}

// =======================================================================

pub fn square_to_uniform_sphere(sample: Vec2) -> Vec3 {
    let z = sample.y.mul_add(2., 1.);
    let r = circ(z);
    let v = 2. * PI * sample.x;
    vec3(r * v.cos(), r * v.sin(), z)
}

pub fn sphere_to_square(p: Vec3) -> Vec2 {
    let phi = p.y.atan2(p.x) * 1. / (2. * PI);
    vec2(if phi < 0. { phi + 1. } else { phi }, (1. - p.z) * 0.5)
}

pub fn square_to_uniform_sphere_pdf(p: Vec3) -> f32 {
    1. / (4. * PI)
}

// =======================================================================

pub fn square_to_uniform_hemisphere(sample: Vec2) -> Vec3 {
    todo!()
}

// =======================================================================
pub fn square_to_cosine_hemisphere(sample: Vec2) -> Vec3 {
    let p = square_to_uniform_disk_concentric(sample);

    let z = (1. - p.length_squared()).sqrt();

    vec3(p.x, p.y, z)
}

pub fn cosine_hemisphere_to_square(v: Vec3) -> Vec2 {
    uniform_disk_to_square_concentric(v.xy())
}

pub fn square_to_cosine_hemisphere_pdf(v: Vec3) -> f32 {
    1. / PI * v.z
}
