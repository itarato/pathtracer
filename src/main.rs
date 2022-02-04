// Source: https://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf

mod cam;
mod defs;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use cam::Cam;
use defs::FloatT;
use hitable::{HitState, Hitable};
use hitable_list::HitableList;
use png::Encoder;
use rand::prelude::*;
use ray::Ray;
use sphere::Sphere;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use vec3::{v3, Upscale, Vec3};

fn color(ray: &Ray, hitable: &dyn Hitable) -> Vec3 {
    let mut hit_state = HitState::default();

    if hitable.hit(ray, 0.0, FloatT::MAX, &mut hit_state) {
        (hit_state.normal + 1.0) * 0.5
    } else {
        let unit_dir = ray.direction().unit();
        let t = (unit_dir.y + 1.0) * 0.5;
        v3!(1.0 - t) + (v3!(0.5, 0.7, 1.0) * t)
    }
}

fn main() {
    let w = 200u32; // Ref: `nx`
    let h = 100u32; // Ref: `ny`

    // SETUP PNG //////////////////////////////////////////////////////////////
    let file_path = "./output/0.png";
    let file = File::create(file_path).unwrap();
    let mut buf_writer = BufWriter::new(file);
    let mut encoder = Encoder::new(&mut buf_writer, w, h);

    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut write_header = encoder.write_header().unwrap();
    let mut stream_writer = write_header.stream_writer().unwrap();
    // END SETUP PNG //////////////////////////////////////////////////////////

    let hitable_list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(v3!(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(v3!(0.0, -100.5, -1.0), 100.0)),
    ];
    let hitlist = HitableList::new(hitable_list);

    let cam = Cam::new();
    let anti_alias_attempt = 16;
    let mut rng = thread_rng();

    // DRAW LOOP //////////////////////////////////////////////////////////////
    for y in (0..h).rev() {
        for x in 0..w {
            let mut c = v3!(0.0);

            for _ in 0..anti_alias_attempt {
                let u = (x as FloatT + rng.gen_range(0.0..1.0)) / (w as FloatT);
                let v = (y as FloatT + rng.gen_range(0.0..1.0)) / (h as FloatT);

                let ray = cam.ray(u, v);
                c += color(&ray, &hitlist);
            }
            c /= anti_alias_attempt as FloatT;

            stream_writer
                .write(&[c.x.upscale(), c.y.upscale(), c.z.upscale()])
                .unwrap();
        }
    }
}
