use crate::{
    defs::FloatT,
    hitable::{HitState, Hitable},
    ray::Ray,
};
use std::vec::Vec;

#[derive(Default)]
pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> Self {
        Self { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: FloatT, t_max: FloatT, state: &mut HitState) -> bool {
        let mut hit_state = HitState::default();
        let mut did_hit = false;
        let mut closest = t_max;

        for e in &self.list {
            if e.hit(ray, t_min, closest, &mut hit_state) {
                did_hit = true;
                closest = hit_state.t;
                *state = hit_state.clone();
            }
        }

        return did_hit;
    }
}
