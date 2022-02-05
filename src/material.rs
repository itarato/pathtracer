use crate::{defs, hitable::HitState, ray::Ray, vec3::Vec3};

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * Vec3::dot(v, n) * 2.0
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_state: &HitState,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray,
        hit_state: &HitState,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = hit_state.p + hit_state.normal + defs::rand_in_unit_sphere();
        *scattered = Ray::new(hit_state.p, target - hit_state.p);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_state: &HitState,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(ray_in.direction().unit(), hit_state.normal);
        *scattered = Ray::new(hit_state.p, reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), hit_state.normal) > 0.0
    }
}
