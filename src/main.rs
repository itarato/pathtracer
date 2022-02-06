mod cam;
mod defs;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use cam::Cam;
use defs::FloatT;
use hitable::{HitState, Hitable};
use hitable_list::HitableList;
use material::{Dialectric, Lambertian, Material, Metal};
use png::Encoder;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};
use vec3::{v3, Upscale, Vec3};

fn color(ray: &Ray, hitable: &dyn Hitable, depth: i32) -> Vec3 {
    let mut hit_state = HitState::default();

    if hitable.hit(ray, 0.001, FloatT::MAX, &mut hit_state) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();

        let hit_state_clone = hit_state.clone();
        if depth < 50
            && hit_state.material.unwrap().scatter(
                &ray,
                &hit_state_clone,
                &mut attenuation,
                &mut scattered,
            )
        {
            attenuation * color(&scattered, hitable, depth + 1)
        } else {
            v3!(0.0)
        }
    } else {
        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y + 1.0) * 0.5;
        v3!(1.0 - t) + (v3!(0.5, 0.7, 1.0) * t)
    }
}

fn main() {
    let w = 2048u32; // Ref: `nx`
    let h = 1024u32; // Ref: `ny`
                     // let w = 512u32; // Ref: `nx`
                     // let h = 512u32; // Ref: `ny`
                     // let w = 256u32; // Ref: `nx`
                     // let h = 128u32; // Ref: `ny`

    // SETUP PNG //////////////////////////////////////////////////////////////
    let file_path = "./output/0.png";
    let file = File::create(file_path).unwrap();
    let mut buf_writer = BufWriter::new(file);
    let mut encoder = Encoder::new(&mut buf_writer, w, h);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Sixteen);

    let mut write_header = encoder.write_header().unwrap();
    let mut stream_writer = write_header.stream_writer().unwrap();
    // END SETUP PNG //////////////////////////////////////////////////////////

    let mat1 = Rc::new(Lambertian::new(v3!(0.1, 0.2, 0.5)));
    let mat2 = Rc::new(Lambertian::new(v3!(0.5, 0.5, 0.5)));
    let mat3 = Rc::new(Metal::new(v3!(0.8, 0.6, 0.2), 0.0));
    let mat4 = Rc::new(Dialectric::new(1.5));

    let mut hitable_list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(v3!(-2.0, 1.0, -1.0), 1.0, mat1.clone())),
        Box::new(Sphere::new(v3!(0.0, -1000.0, 0.0), 1000.0, mat2.clone())),
        Box::new(Sphere::new(v3!(2.0, 1.0, -1.0), 1.0, mat3.clone())),
        Box::new(Sphere::new(v3!(0.0, 1.0, -1.0), 1.0, mat4.clone())),
        // Trick to make a thick glass.
        // Box::new(Sphere::new(v3!(0.0, 1.0, -1.0), -0.9, mat4.clone())),
    ];

    let mut rng = thread_rng();

    let rand_spread_h: FloatT = 8.0;
    for _ in 0..128 {
        let rand_spread_r = 0.2 as FloatT..0.3 as FloatT;
        let rand_r = rng.gen_range(rand_spread_r.clone());

        let mat: Rc<dyn Material> = match rng.gen_range(0..3) {
            0 => Rc::new(Lambertian::new(v3!(
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0),
                rng.gen_range(0.0..1.0)
            ))),
            1 => Rc::new(Metal::new(
                v3!(
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0),
                    rng.gen_range(0.0..1.0)
                ),
                rng.gen_range(0.0..1.0),
            )),
            _ => Rc::new(Dialectric::new(rng.gen_range(0.0..4.0))),
        };

        hitable_list.push(Box::new(Sphere::new(
            v3!(
                rng.gen_range(-rand_spread_h..rand_spread_h),
                rand_r,
                rng.gen_range(-rand_spread_h..rand_spread_h) - 1.0
            ),
            rand_r,
            mat,
        )));
    }

    let hitlist = HitableList::new(hitable_list);

    let lookfrom = v3!(4.0, 1.75, 1.25);
    let lookat = v3!(0.0, 0.0, -1.0);
    let cam = Cam::new(
        lookfrom,
        lookat,
        v3!(1.0, 2.0, 0.4),
        80.0,
        w as FloatT / h as FloatT,
        0.001,
        (lookfrom - lookat).len(),
    );
    let anti_alias_attempt = 32;

    // DRAW LOOP //////////////////////////////////////////////////////////////
    for y in (0..h).rev() {
        for x in 0..w {
            let mut c = v3!(0.0);

            for _ in 0..anti_alias_attempt {
                let u = (x as FloatT + rng.gen_range(0.0..1.0)) / (w as FloatT);
                let v = (y as FloatT + rng.gen_range(0.0..1.0)) / (h as FloatT);

                let ray = cam.ray(u, v);
                c += color(&ray, &hitlist, 0);
            }
            c /= anti_alias_attempt as FloatT;

            let r = c.x.sqrt().upscale();
            let g = c.y.sqrt().upscale();
            let b = c.z.sqrt().upscale();

            stream_writer
                .write(&[
                    (r >> 8) as u8,
                    (r | 0xff) as u8,
                    (g >> 8) as u8,
                    (g | 0xff) as u8,
                    (b >> 8) as u8,
                    (b | 0xff) as u8,
                ])
                .unwrap();
        }
    }
}
