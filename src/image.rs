use contracts::*;

use crate::camera::Camera;
use crate::color::Color;
use crate::ray::Ray;
use crate::scene::sphere::Sphere;
use crate::scene::Scene;
use crate::vec3::{self, RandomInUnitSphere, Vec3};
use indicatif::ParallelProgressIterator;

use rayon::prelude::*;

pub mod samplers;

const BOUNCES: u8 = 255;

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
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width,
            height,
            aspect_ratio: width as f64 / height as f64,
            pixels: vec![Color::black(); (width * height) as usize],
        }
    }

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
        let ray_gen = cam
            .get_rays(width, height)
            .map(|ray| samplers::sample_cluster::SampleCluster::from_camera_ray(cam.clone(), ray));

        let mut image = Image::noise(&mut random::default(1337), width, height);
        let samples_clusters = ray_gen.collect::<Vec<_>>();
        image.pixels = samples_clusters
            .into_par_iter()
            .progress_count(image.pixels.len() as u64)
            .map(|cluster| {
                let mut rand = random::default(1337);
                // as random::Source
                let mut rand: &mut dyn random::Source = &mut rand;
                let len = cluster.len() as i64;
                let color = cluster.fold(((0f64, 0f64, 0f64), len), |acc, ray| {
                    let color =
                        (1..=BOUNCES).fold(((0.0, 0.0, 0.0), ray, -1i64, true), |acc, _| {
                            let color = &acc.0;
                            let ray = &acc.1;
                            let hits = acc.2;
                            let has_hit = acc.3;
                            if !has_hit {
                                return acc;
                            }

                            let hit = scene.intersect(&ray);
                            if let Some(hit) = hit {
                                let color = (
                                    (hit.material.color.x * hit.material.albedo
                                        + hit.material.roughness * rand.read_f64())
                                    .clamp(0.0, 255.0)
                                        + color.0,
                                    (hit.material.color.y * hit.material.albedo).clamp(0.0, 255.0)
                                        + hit.material.roughness * color.1,
                                    (hit.material.color.z * hit.material.albedo
                                        + hit.material.roughness * rand.read_f64())
                                    .clamp(0.0, 255.0)
                                        + color.2,
                                );

                                let hit_point = ray.at(hit.t);
                                // let hit_normal = hit.normal;
                                let hit_normal = RandomInUnitSphere::new(rand)
                                    .map(|normal| {
                                        let mut normal = normal;
                                        if normal.dot(&ray.dir) > 0.0 {
                                            normal = -normal;
                                        }
                                        normal
                                    })
                                    .find(|normal| {
                                        let mut normal = normal;
                                        if normal.dot(&ray.dir) > 0.0 {
                                            normal = -normal;
                                        }
                                        normal.dot(&ray.dir) < 0.0
                                    })
                                    .unwrap();
                                let mut new_ray = Ray::new(hit_point, hit_normal);
                                new_ray.origin = new_ray.origin + new_ray.dir * 0.0001;
                                (color, new_ray, hits + 1, true)
                            } else {
                                (*color, ray.clone(), hits, false)
                            }
                        });

                    let acc_color = acc.0;
                    let acc_hits = acc.1;
                    (
                        (
                            color.0 .0 + acc_color.0,
                            color.0 .1 + acc_color.1,
                            color.0 .2 + acc_color.2,
                        ),
                        color.2 + acc_hits,
                    )
                });
                let len = color.1;
                let color = color.0;
                Color {
                    r: (color.0 / len as f64) as u8,
                    g: (color.1 / len as f64) as u8,
                    b: (color.2 / len as f64) as u8,
                }
            })
            .collect::<Vec<Color>>();
        image
    }

    #[invariant(self.pixels.len() == (self.width * self.height) as usize)]
    pub fn save_to_file(&self, filename: &str) -> anyhow::Result<()> {
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
    fn f64_to_u8() {
        assert_eq!(-500.0 as u8, 0);
        assert_eq!(-255.0 as u8, 0);
        assert_eq!(-1.0 as u8, 0);
        assert_eq!(0.0 as u8, 0);
        assert_eq!(1.0 as u8, 1);
        assert_eq!(255.0 as u8, 255);
        assert_eq!(500.0 as u8, 255);
    }

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
    #[ignore]
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

