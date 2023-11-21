use contracts::*;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl From<&[u8]> for Vec3 {
    fn from(pixel: &[u8]) -> Self {
        Vec3 {
            x: pixel[0] as f32 / 255.0,
            y: pixel[1] as f32 / 255.0,
            z: pixel[2] as f32 / 255.0,
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

#[derive(Debug)]
struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Vec3>,
}

impl Image {
    #[ensures(ret.pixels.len() == (ret.width * ret.width) as usize)]
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rand = random::default(1337);
    let image = Image::noise(&mut rand);
    image.save_to_file("noise.ppm")?;
    println!("Hello, world!");

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
    }
}
