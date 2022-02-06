use crate::{
    defs::{rand_in_unit_disk, FloatT},
    ray::Ray,
    vec3::Vec3,
};
use std::f32::consts::PI;

pub struct Cam {
    origin: Vec3,
    bottom_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_rad: FloatT,
}

impl Cam {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: FloatT,
        aspect: FloatT,
        aperture: FloatT,
        focus_dist: FloatT,
    ) -> Self {
        let theta = vfov * PI / 180.0;
        let half_h = (theta / 2.0).tan();
        let half_w = aspect * half_h;

        let w = (lookfrom - lookat).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        Self {
            origin: lookfrom,
            bottom_left_corner: lookfrom
                - u * half_w * focus_dist
                - v * half_h * focus_dist
                - w * focus_dist,
            horizontal: u * half_w * 2.0 * focus_dist,
            vertical: v * half_h * 2.0 * focus_dist,
            u,
            v,
            lens_rad: aperture / 2.0,
        }
    }

    pub fn ray(&self, u: FloatT, v: FloatT) -> Ray {
        let rd = rand_in_unit_disk() * self.lens_rad;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.bottom_left_corner + (self.horizontal * u) + (self.vertical * v)
                - self.origin
                - offset,
        )
    }
}
