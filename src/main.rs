use std::time::SystemTime;

use raytracer::surface::surfaces::bvh_node::BvhNode;

fn main() {
    let start = SystemTime::now();
    let (world, camera) = raytracer::scenes::random_balls::scene();
    let world = BvhNode::new_from_list(world.into_objects());

    camera.render(world.as_ref());
    let end = SystemTime::now();
    eprintln!("Took: {:?}", end.duration_since(start).unwrap());
}