use std::io::{self, Write};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let header = format!("P3\n{} {}\n256\n", image_width, image_height);

    println!("{}", header);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        io::stderr().flush().unwrap();

        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\rScanlines remaining: 0");
    io::stderr().flush().unwrap();
    eprintln!("\nDone!")
}