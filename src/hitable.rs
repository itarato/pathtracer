use crate::defs::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct HitState {
    pub t: FloatT,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Rc<dyn Material>>,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: FloatT, t_max: FloatT, state: &mut HitState) -> bool;
}
