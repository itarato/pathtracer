// Source: https://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf

mod defs;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use defs::FloatT;
use hitable::{HitState, Hitable};
use hitable_list::HitableList;
use png::Encoder;
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
    let w = 200u32;
    let h = 100u32;
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

    let bottom_left_corner = v3!(-2.0, -1.0, -1.0);
    let horizontal = v3!(4.0, 0.0, 0.0);
    let vertical = v3!(0.0, 2.0, 0.0);
    let origin = v3!(0.0);

    let hitable_list: Vec<Box<dyn Hitable>> = vec![
        Box::new(Sphere::new(v3!(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(v3!(0.0, -100.5, -1.0), 100.0)),
    ];
    let hitlist = HitableList::new(hitable_list);

    for y in (0..h).rev() {
        for x in 0..w {
            let u = (x as FloatT) / (w as FloatT);
            let v = (y as FloatT) / (h as FloatT);

            let ray = Ray::new(
                origin,
                bottom_left_corner + (horizontal * u) + (vertical * v),
            );
            let c = color(&ray, &hitlist);
            stream_writer
                .write(&[c.x.upscale(), c.y.upscale(), c.z.upscale()])
                .unwrap();
        }
    }
}
