use crate::defs::*;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct HitState {
    pub t: FloatT,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: FloatT, t_max: FloatT, state: &mut HitState) -> bool;
}
