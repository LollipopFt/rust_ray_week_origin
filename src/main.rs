mod vec3;
use vec3::Vec3;

fn main() {
    // image:
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;

    // render:
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaning: {}", j);
        for i in 0..IMAGE_WIDTH {
            let color = Vec3::new(
                (i as f64)/(IMAGE_WIDTH as f64 - 1.0),
                (j as f64)/(IMAGE_HEIGHT as f64 - 1.0),
                0.25
            );
            color.prt();
        }
    }
    eprintln!("\nDone.");
}