use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let (world, camera) = raytracer::scenes::random_balls::scene();
    camera.render(&world);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    eprintln!("Took: {:?}", end - start);
}
