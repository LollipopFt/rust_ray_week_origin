#![allow(dead_code)]

use std::io::{self, Write};
mod vec3;
mod ray;
use vec3::Vec3;
use ray::Ray;

fn main() {
    let mut handle = io::BufWriter::new(io::stdout());
    // genericimage(&mut handle);
    // image:
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = ((IMAGE_WIDTH as f64)/ASPECT_RATIO) as u16;
    // camera:
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;
    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = origin.sub(&horizontal.div(2.0)).sub(&vertical.div(2.0))
        .sub(&Vec3::new(0.0, 0.0, FOCAL_LENGTH));
    println!("{:?}", lower_left_corner);
    // render:
    writeln!(handle, "P3\n{} {}\n255", 256, 256).ok();
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64)/(IMAGE_WIDTH as f64 - 1.0);
            let v = (j as f64)/(IMAGE_HEIGHT as f64 - 1.0);
            println!("{} {}", u, v);
            let ray = Ray::new(origin, lower_left_corner.add(&horizontal.mul(u)).add(&vertical.mul(v)));
            println!("{:?}", ray);
            let pix_colour = ray_colour(&ray);
            pix_colour.write_colour(&mut handle);
        }
    }
}
// function to generate image1
// fn genericimage(handle: &mut io::BufWriter<io::Stdout>) {
//     const IMAGE_WIDTH: f64 = 256.0;
//     const IMAGE_HEIGHT: f64 = 256.0;
//     writeln!(handle, "P3\n{} {}\n255", 256, 256).ok();
//     for j in (0..IMAGE_HEIGHT as u16).rev() {
//         for i in 0..IMAGE_WIDTH as u16 {
//             let pix_colour = Vec3::new((i as f64)/(IMAGE_WIDTH-1.0), (j as f64)/(IMAGE_HEIGHT-1.0), 0.25);
//             pix_colour.write_colour(handle);
//         }
//     }
// }

fn ray_colour(ray: &Ray) -> Vec3 {
    let unit_dir: Vec3 = ray.dir().unit_vec();
    let t = 0.5*(unit_dir.y()+1.0);
    Vec3::new(1.0, 1.0, 1.0).mul(t).add(&Vec3::new(0.5, 0.7, 1.0).mul(t))
}