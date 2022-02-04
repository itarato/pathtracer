use crate::defs::*;
use crate::hitable::*;
use crate::ray::Ray;
use crate::vec3::{v3, Vec3};

struct Sphere {
    center: Vec3,
    r: FloatT,
}

impl Sphere {
    pub fn new(center: Vec3, r: FloatT) -> Self {
        Self { center, r }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: FloatT, t_max: FloatT, state: &mut HitState) -> bool {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        let b = Vec3::dot(oc, ray.direction());
        let c = Vec3::dot(oc, oc) - (self.r * self.r);
        let discriminant = (b * b) - (a * c);

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                state.t = temp;
                state.p = ray.point_at(temp);
                state.normal = (state.p - self.center) / self.r;
                return true;
            }

            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                state.t = temp;
                state.p = ray.point_at(temp);
                state.normal = (state.p - self.center) / self.r;
                return true;
            }
        }
        false
    }
}
