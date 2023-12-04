pub mod camera;
pub mod color;
pub mod image;
pub mod ray;
pub mod scene;
pub mod vec3;

use anyhow::Result;
use image::Image;
use vec3::{Pnt3, UnitVec3, Vec3};

#[macro_use]
extern crate my_macro;

fn main() -> Result<()> {
    let cam = camera::Camera::look_at(
        Vec3 {
            x: 100.0,
            y: 50.0,
            z: 10.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );
    // cam.right.z = 1.0;
    // println!("{:?}", cam);
    let mut scene = scene::Scene::new();
    scene.add_sphere(scene::sphere::Sphere::new(
        Pnt3 {
            x: 0.0,
            y: 0.0,
            z: -50.0,
        },
        33.3,
    ));
    scene.add_cube(scene::cube::Cube::new(
        Pnt3 {
            x: -50.0,
            y: 0.0,
            z: 0.0,
        },
        Pnt3 {
            x: 50.0,
            y: 50.0,
            z: 50.0,
        },
    ));
    scene.add_plane(scene::plane::Plane::new(
        Pnt3 {
            x: 0.0,
            y: -50.0,
            z: 0.0,
        },
        UnitVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    ));
    // Show Coordinate system origin
    scene.add_line(scene::line::Line::new(
        Pnt3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        UnitVec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        50.0,
        100.0,
    ));
    scene.add_line(scene::line::Line::new(
        Pnt3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        UnitVec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        50.0,
        100.0,
    ));
    scene.add_line(scene::line::Line::new(
        Pnt3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        UnitVec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        50.0,
        100.0,
    ));
    let image = Image::gen_image(&cam, &scene, 1600, 900);
    image.save_to_file("/tmp/run_render.ppm")?;
    Ok(())
}
