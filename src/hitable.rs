use super::defs::*;
use super::vec3::Vec3;
use crate::ray::Ray;

pub struct HitState {
    pub t: FloatT,
    pub p: Vec3,
    pub normal: Vec3,
}

impl HitState {
    pub fn new(t: FloatT, p: Vec3, normal: Vec3) -> Self {
        Self { t, p, normal }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: FloatT, t_max: FloatT, state: &mut HitState) -> bool;
}
