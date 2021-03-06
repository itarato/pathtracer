use crate::vec3::{v3, Vec3};
use rand::{thread_rng, Rng};

pub type FloatT = f32;

pub fn rand_in_unit_sphere() -> Vec3 {
    let mut p;
    let mut rng = thread_rng();

    loop {
        p = v3!(rng.gen(), rng.gen(), rng.gen()) * 2.0 - v3!(1.0);

        if p.squared_len() <= 1.0 {
            break;
        }
    }

    p
}

pub fn rand_in_unit_disk() -> Vec3 {
    let mut p;
    let mut rng = thread_rng();

    loop {
        p = v3!(rng.gen(), rng.gen(), 0.0) * 2.0 - v3!(1.0, 1.0, 0.0);

        if Vec3::dot(p, p) <= 1.0 {
            break;
        }
    }

    p
}
