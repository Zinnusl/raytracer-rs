pub mod camera;
pub mod color;
pub mod image;
pub mod interval;
pub mod material;
pub mod ray;
pub mod scene;
pub mod vec3;

use anyhow::Result;
use image::Image;
use vec3::{Pnt3, UnitVec3, Vec3};

#[macro_use]
extern crate my_macro;

fn main() -> Result<()> {
    let mut cam = camera::Camera::look_at(
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
        66.6,
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

    for i in 0..1 {
        scene.add_sphere(scene::sphere::Sphere::new(
            Pnt3 {
                x: 0.0,
                y: 0.0,
                z: -50.0 + i as f64 * 10.0,
            },
            5.0,
        ));

        cam.center.z += 10.0;
        let image = Image::gen_image(&cam, &scene, 1600, 900);
        image.save_to_file(format!("/tmp/run_render.ppm").as_str())?;
    }
    Ok(())
}
