use std::io::{self, Write};

mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_dir: Vec3 = ray.dir.unit_vec();
    let t: f64 = 0.5 * (unit_dir.y()+1.0);
    Vec3::new(1.0, 1.0, 1.0)*(1.0-t)+Vec3::new(0.5, 0.7, 1.0)*t
}

fn main() {
    let mut handle = io::BufWriter::new(io::stdout());
    let mut handlerr = io::BufWriter::new(io::stderr());
    // image:
    const ASPECT_RATIO: f64 = 16.0/9.0;
    const IMAGE_WIDTH: u16 = 400;
    const IMAGE_HEIGHT: u16 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u16;

    // camera:
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // render:
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        writeln!(handlerr, "\rScanlines remaning: {}", j).ok();
        for i in 0..IMAGE_WIDTH {
            let u: f64 = i as f64 / (IMAGE_WIDTH as f64 - 1.0);
            let v: f64 = j as f64 / (IMAGE_HEIGHT as f64 - 1.0);
            let r: Ray = Ray::new(origin, lower_left_corner+horizontal*u+vertical*v-origin);
            let pixel_color = ray_color(&r);
            pixel_color.prt(&mut handle);
        }
    }
    writeln!(handlerr, "\nDone.").ok();
}