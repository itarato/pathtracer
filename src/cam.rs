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
    pub fn new() -> Self {
        Self {
            origin: v3!(0.0),
            bottom_left_corner: v3!(-2.0, -1.0, -1.0),
            horizontal: v3!(4.0, 0.0, 0.0),
            vertical: v3!(0.0, 4.0, 0.0),
        }
    }

    pub fn ray(&self, u: FloatT, v: FloatT) -> Ray {
        Ray::new(
            self.origin,
            self.bottom_left_corner + (self.horizontal * u) + (self.vertical * v),
        )
    }
}
