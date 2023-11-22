use contracts::*;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: -(self.x * other.z - self.z * other.x),
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x + other.x)]
    #[ensures(ret.y == self.y + other.y)]
    #[ensures(ret.z == self.z + other.z)]
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x - other.x)]
    #[ensures(ret.y == self.y - other.y)]
    #[ensures(ret.z == self.z - other.z)]
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;

    #[ensures(ret.x == self.x * other)]
    #[ensures(ret.y == self.y * other)]
    #[ensures(ret.z == self.z * other)]
    fn mul(self, other: f64) -> Self {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl From<&[u8]> for Vec3 {
    fn from(pixel: &[u8]) -> Self {
        Vec3 {
            x: pixel[0] as f64 / 255.0,
            y: pixel[1] as f64 / 255.0,
            z: pixel[2] as f64 / 255.0,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            (self.x * 255.0) as u8,
            (self.y * 255.0) as u8,
            (self.z * 255.0) as u8
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec3>,
}

impl Image {
    #[ensures(ret.pixels.len() == (ret.width * ret.height) as usize)]
    fn noise<RandGen>(rand: &mut RandGen) -> Image
    where
        RandGen: random::Source,
    {
        Image {
            width: 1024,
            height: 1024,
            pixels: rand
                .iter()
                .take(1024 * 1024 * 3)
                .collect::<Vec<u8>>()
                .chunks(3)
                .map(|pixel| pixel.into())
                .collect::<Vec<Vec3>>(),
        }
    }

    fn save_to_file(&self, filename: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(filename)?;
        let data = self
            .pixels
            .chunks((self.width * 3) as usize)
            .fold(String::new(), |acc, row| {
                acc + row
                    .chunks(3)
                    .fold(String::new(), |acc, pixel| format!("{} {} ", acc, pixel[0]))
                    .as_str()
                    + "\n"
            });
        file.write_all(format!("P6\n{} {}\n255\n", self.width, self.height).as_bytes())?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}

impl Ray {
    #[ensures(ret.origin == origin)]
    #[ensures(ret.dir == dir)]
    fn new(origin: Vec3, dir: Vec3) -> Ray {
        Ray { origin, dir }
    }

    #[ensures(ret == self.origin + self.dir * t)]
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }

    #[ensures(ret == None || ret.unwrap() > 0.0)]
    fn intersect(&self, sphere: &Sphere) -> Option<f64> {
        let oc = self.origin - sphere.center;
        let a = self.dir.dot(self.dir);
        let b = 2.0 * oc.dot(self.dir);
        let c = oc.dot(oc) - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * a * c;
        let t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t >= 0.0 {
            Some(t)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    #[ensures(ret.center == center)]
    #[ensures(ret.radius == radius)]
    fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut rand = random::default(1337);
        let image = Image::noise(&mut rand);
        assert_eq!(image.width, 1024);
        assert_eq!(image.height, 1024);
        assert_eq!(image.pixels.len(), 1024 * 1024);
    }

    #[test]
    fn save_to_file() {
        let mut rand = random::default(1337);
        let image = Image::noise(&mut rand);
        image.save_to_file("/tmp/noise.ppm").unwrap();

        // Check if file size is ok
        let metadata = std::fs::metadata("/tmp/noise.ppm").unwrap();
        assert_eq!(metadata.len(), 4093527);
    }

    #[test]
    fn vec3_add() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a + b;
        assert_eq!(c.x, 5.0);
        assert_eq!(c.y, 7.0);
        assert_eq!(c.z, 9.0);

        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: -4.0,
            y: -5.0,
            z: -6.0,
        };
        let c = a + b;
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, -3.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn vec3_cross() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a.cross(b);
        assert_eq!(c.x, -3.0);
        assert_eq!(c.y, 6.0);
        assert_eq!(c.z, -3.0);
    }

    #[test]
    fn vec3_mul_scalar() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = 2.0;
        let c = a * b;
        assert_eq!(c.x, 2.0);
        assert_eq!(c.y, 4.0);
        assert_eq!(c.z, 6.0);
    }

    #[test]
    fn vec3_dot() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 32.0);

        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 0.0);

        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let c = a.dot(b);
        assert_eq!(c, 1.0);
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(
            Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        );
        let point = ray.at(2.0);
        assert_eq!(point.x, 9.0);
        assert_eq!(point.y, 12.0);
        assert_eq!(point.z, 15.0);
    }

    #[test]
    fn sphere_new() {
        let sphere = Sphere {
            center: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 4.0,
        };
        assert_eq!(sphere.center.x, 1.0);
        assert_eq!(sphere.center.y, 2.0);
        assert_eq!(sphere.center.z, 3.0);
        assert_eq!(sphere.radius, 4.0);
    }

    #[test]
    fn ray_sphere_intersection() {
        let sphere = Sphere {
            center: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let ray = Ray::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );
        let intersection = ray.intersect(&sphere);
        assert_eq!(intersection, Some(1.0));

        let ray = Ray::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        );
        let intersection = ray.intersect(&sphere);
        assert_eq!(intersection, None);

        let ray = Ray::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );

        let intersection = ray.intersect(&sphere);
        assert_eq!(intersection, None);
    }
}
