use std::io::{self, Write};
use std::time::UNIX_EPOCH;
use std::time::SystemTime;
use raytracer::ray::color::{color, write_col};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let image_width = 256;
    let image_height = 256;

    let header = format!("P3\n{} {}\n256\n", image_width, image_height);

    println!("{}", header);

    for y in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - y);
        io::stderr().flush().unwrap();

        for x in 0..image_width {
            let col = color::new(x as f64 / (image_width - 1) as f64, y as f64 / (image_height - 1) as f64, 1.0);
            write_col(&col);
        }
    }
    eprint!("\rDone!                 \n");
    io::stderr().flush().unwrap();
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let time = end - start;
    eprintln!("Took: {:?}", time)
}