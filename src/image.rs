use contracts::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    width: u32,
    height: u32,
    aspect_ratio: f64,
    pixels: Vec<Color>,
}

impl Image {
    #[requires(width > 0)]
    #[requires(height > 0)]
    #[ensures(ret.pixels.len() == (ret.width * ret.height) as usize)]
    pub fn noise<RandGen>(rand: &mut RandGen, width: u32, height: u32) -> Image
    where
        RandGen: random::Source,
    {
        Image {
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
            pixels: rand
                .iter()
                .take((width * height * 3u32) as usize)
                .collect::<Vec<u8>>()
                .chunks(3)
                .map(|pixel| pixel.into())
                .collect::<Vec<Color>>(),
        }
    }

    #[requires(width > 0)]
    #[requires(height > 0)]
    #[ensures(ret.pixels.len() == (ret.width * ret.height) as usize)]
    pub fn gen_image(cam: &Camera, scene: &Scene, width: u32, height: u32) -> Image {
        // ray_gen: &mut dyn Iterator<Item = Ray>
        let ray_gen = (0..width * height).map(|i| {
            let x = (i % width) as f64;
            let y = (i / height) as f64;
            // let right = cam.right * (2.0 * (x + 0.5) / width as f64 - 1.0) * cam.aspect_ratio;
            let right = cam.right * (2.0 * (x + 0.5) / height as f64 - 1.0);
            let up = cam.up * (2.0 * (y + 0.5) / width as f64 - 1.0);
            let pnt = cam.center + right * cam.focal_length + up * cam.focal_length;
            Ray::new(pnt, cam.up.cross(cam.right).normalize().unwrap())
        });
        let mut image = Image::noise(&mut random::default(1337), width, height);
        for (i, ray) in ray_gen.enumerate() {
            if let Some((_t, normal)) = scene.intersect(&ray) {
                let color = Color {
                    r: ((normal.x * 255.0 + (255 as f64) * normal.dot(ray.dir).exp()) / 2.0) as u8,
                    g: ((normal.y * 255.0 + (255 as f64) * normal.dot(ray.dir).exp()) / 2.0) as u8,
                    b: ((normal.z * 255.0 + (255 as f64) * normal.dot(ray.dir).exp()) / 2.0) as u8,
                };
                image.pixels[i] = color;
            }
        }
        image
    }

    #[invariant(self.pixels.len() == (self.width * self.height) as usize)]
    pub fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;
        let data = self.pixels.iter().fold(String::new(), |acc, row| {
            acc + "\n" + format!("{}", row).as_str()
        });
        file.write_all(format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::vec3::{Pnt3, UnitVec3};
    use crate::{scene, vec3};

    use super::*;

    #[test]
    fn create_noise_image() {
        let mut rand = random::default(1337);
        let image = Image::noise(&mut rand, 1600, 900);
        assert_eq!(image.width, 1600);
        assert_eq!(image.height, 900);
        assert_eq!(image.pixels.len(), 1600 * 900);
    }

    #[test]
    fn save_noise_to_file() {
        let mut rand = random::default(1337);
        let image = Image::noise(&mut rand, 1600, 900);
        image.save_to_file("/tmp/noise.ppm").unwrap();

        // Check if file size is ok
        let metadata = std::fs::metadata("/tmp/noise.ppm").unwrap();
        // greater than 1600 * 900 * 3
        assert!(metadata.len() > 1600 * 900 * 3);
    }

    #[test]
    fn save_render_to_file() {
        let cam = Camera {
            focal_length: 1.0,
            center: Pnt3 {
                x: 350.0,
                y: 350.0,
                z: 0.0,
            },
            up: UnitVec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            right: UnitVec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let mut scene = scene::Scene::new();
        scene.add_sphere(scene::sphere::Sphere::new(
            vec3::Pnt3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            100.0,
        ));
        scene.add_cube(scene::cube::Cube::new(
            vec3::Pnt3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            vec3::Pnt3 {
                x: 50.0,
                y: 50.0,
                z: 50.0,
            },
        ));
        let image = Image::gen_image(&cam, &scene, 1600, 900);
        image.save_to_file("/tmp/render.ppm").unwrap();

        // Check if file size is ok
        // let metadata = std::fs::metadata("/tmp/render.ppm").unwrap();
        // assert_eq!(metadata.len(), 4093527);
    }
}
