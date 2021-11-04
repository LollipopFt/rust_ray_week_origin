fn main() {
    // image:
    const IMAGE_WIDTH: u16 = 256;
    const IMAGE_HEIGHT: u16 = 256;
    // render:
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let r = (i as f64)/(IMAGE_WIDTH as f64 - 1.0);
            let g = (j as f64)/(IMAGE_HEIGHT as f64 - 1.0);
            let b: f64 = 0.25;

            let ir = (255.999 * r) as u16;
            let ig = (255.999 * g) as u16;
            let ib = (255.999 * b) as u16;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}