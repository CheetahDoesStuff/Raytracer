use std::time::SystemTime;

use colored::Colorize;
use raytracer::surface::surfaces::bvh_node::BvhNode;

fn main() {
    eprintln!("Raytracer by CheetahDoesStuff");
    eprintln!("Features:");
    #[cfg(feature = "threaded")]
    eprintln!("  - Multithreaded (threaded): {}", "ENABLED".green());
    #[cfg(not(feature = "threaded"))]
    eprintln!("  - Multithreaded (threaded): {}", "DISABLED".red());
    #[cfg(feature = "denoise")]
    eprintln!("  - Image denoising (denoise): {}", "ENABLED".green());
    #[cfg(not(feature = "denoise"))]
    eprintln!("  - Image denoising (denoise): {}", "DISABLED".red());

    eprintln!();

    let start = SystemTime::now();
    let (world, camera) = raytracer::scenes::simple::scene();
    let world = BvhNode::new_from_list(world.into_objects());

    camera.render(world.as_ref());
    let end = SystemTime::now();
    eprintln!("Took: {:?}", end.duration_since(start).unwrap());
}