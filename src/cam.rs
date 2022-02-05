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
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: FloatT, aspect: FloatT) -> Self {
        let theta = vfov * PI / 180.0;
        let half_h = (theta / 2.0).tan();
        let half_w = aspect * half_h;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: lookfrom,
            bottom_left_corner: lookfrom - u * half_w - v * half_h - w,
            horizontal: u * half_w * 2.0,
            vertical: v * half_h * 2.0,
        }
    }

    pub fn ray(&self, u: FloatT, v: FloatT) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left_corner + (self.horizontal * u) + (self.vertical * v) - self.origin,
        )
    }
}
