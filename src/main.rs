use std::time::SystemTime;

use colored::Colorize;
use raytracer::surface::surfaces::bvh_node::BvhNode;

fn main() {
    println!("Raytracer by {}", "CheetahDoesStuff".blue());
    println!("Features:");
    #[cfg(feature = "threaded")]
    println!("  - Multithreaded (threaded): {}", "ENABLED".green());
    #[cfg(not(feature = "threaded"))]
    println!("  - Multithreaded (threaded): {}", "DISABLED".red());
    #[cfg(feature = "denoise")]
    println!("  - Image denoising (denoise): {}", "ENABLED".green());
    #[cfg(not(feature = "denoise"))]
    println!("  - Image denoising (denoise): {}", "DISABLED".red());

    println!();

    let start = SystemTime::now();
    let (world, camera) = raytracer::scenes::earth::scene();
    let world = BvhNode::new_from_list(world.into_objects());

    camera.render(world.as_ref());
    let end = SystemTime::now();
    println!("Took: {}", format!("{:?}", end.duration_since(start).unwrap()).blue());
}