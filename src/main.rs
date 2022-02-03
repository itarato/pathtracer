// Source: https://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf

use defs::FloatT;
use png::Encoder;
use ray::Ray;
use std::{
    fs::File,
    io::{BufWriter, Write},
};
use vec3::{Upscale, Vec3};

mod defs;
mod ray;
mod vec3;

fn color(ray: &Ray) -> Vec3 {
    let unit_dir = ray.direction().unit();
    let t = (unit_dir.y + 1.0) * 0.5;
    Vec3::unif(1.0 - t) + (Vec3::new(0.5, 0.7, 1.0) * t)
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

    let bottom_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::unif(0.0);

    for y in (0..h).rev() {
        for x in 0..w {
            let u = (x as FloatT) / (w as FloatT);
            let v = (y as FloatT) / (h as FloatT);

            let ray = Ray::new(
                origin,
                bottom_left_corner + (horizontal * u) + (vertical * v),
            );
            let c = color(&ray);
            stream_writer
                .write(&[c.x.upscale(), c.y.upscale(), c.z.upscale()])
                .unwrap();
        }
    }
}
