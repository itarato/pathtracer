use rand::{thread_rng, Rng};

use crate::{
    defs::{self, rand_in_unit_sphere, FloatT},
    hitable::HitState,
    ray::Ray,
    vec3::{v3, Vec3},
};

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * Vec3::dot(v, n) * 2.0
}

fn refract(v: Vec3, n: Vec3, ni_over_nt: FloatT, refracted: &mut Vec3) -> bool {
    let uv = v.unit();
    let dt = Vec3::dot(uv, n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = (uv - n * dt) * ni_over_nt - n * discriminant.sqrt();
        true
    } else {
        false
    }
}

fn schlick(cos: FloatT, ref_idx: FloatT) -> FloatT {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
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
    fuzz: FloatT,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: FloatT) -> Self {
        debug_assert!(fuzz <= 1.0);

        Self { albedo, fuzz }
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
        *scattered = Ray::new(hit_state.p, reflected + rand_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), hit_state.normal) > 0.0
    }
}

pub struct Dialectric {
    ref_idx: FloatT,
}

impl Dialectric {
    pub fn new(ref_idx: FloatT) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        hit_state: &HitState,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let reflected = reflect(ray_in.direction(), hit_state.normal);
        let ni_over_nt;

        *attenuation = v3!(1.0);
        let mut refracted = Vec3::default();

        let reflect_prob;
        let cos;

        if Vec3::dot(ray_in.direction(), hit_state.normal) > 0.0 {
            outward_normal = -hit_state.normal;
            ni_over_nt = self.ref_idx;
            cos = self.ref_idx * Vec3::dot(ray_in.direction(), hit_state.normal)
                / ray_in.direction().len();
        } else {
            outward_normal = hit_state.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cos = -Vec3::dot(ray_in.direction(), hit_state.normal) / ray_in.direction().len();
        }

        if refract(
            ray_in.direction(),
            outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            reflect_prob = schlick(cos, self.ref_idx);
        } else {
            *scattered = Ray::new(hit_state.p, reflected);
            reflect_prob = 1.0;
        }

        let mut rng = thread_rng();
        if rng.gen_range(0.0..1.0) < reflect_prob {
            *scattered = Ray::new(hit_state.p, reflected);
        } else {
            *scattered = Ray::new(hit_state.p, refracted);
        }

        true
    }
}
