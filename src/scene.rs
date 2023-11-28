use crate::ray;

// Using Macro, modify struct to add a container for each type of hitable from folder scene/*.rs
//
#[add_containers_from_files("/home/zinnusl/raytracer-rs/src/scene/*.rs")]
struct Scene {}
