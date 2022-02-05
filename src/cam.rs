use std::f32::consts::PI;

use crate::{
    defs::FloatT,
    ray::Ray,
    vec3::{v3, Vec3},
};

pub struct Cam {
    origin: Vec3,
    bottom_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Cam {
    pub fn new(vfov: FloatT, aspect: FloatT) -> Self {
        let theta = vfov * PI / 180.0;
        let half_h = (theta / 2.0).tan();
        let half_w = aspect * half_h;

        Self {
            origin: v3!(0.0),
            bottom_left_corner: v3!(-half_w, -half_h, -1.0),
            horizontal: v3!(2.0 * half_w, 0.0, 0.0),
            vertical: v3!(0.0, 2.0 * half_h, 0.0),
        }
    }

    pub fn ray(&self, u: FloatT, v: FloatT) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left_corner + (self.horizontal * u) + (self.vertical * v),
        )
    }
}
