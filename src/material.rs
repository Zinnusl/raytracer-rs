use contracts::*;

use crate::vec3::Vec3;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    /// Color is the fraction of light reflected by a surface.
    /// 0.0 is black, 255.0 is white.
    pub color: Vec3,
    /// Albedo is the fraction of light reflected by a surface.
    /// 0.0 is black, 1.0 is white.
    pub albedo: f64,
    /// Roughness is the fraction of light reflected in a random direction.
    /// 0.0 is smooth, 1.0 is rough.
    pub roughness: f64,
    /// Refractive index of the material.
    /// 1.0 is air, 1.5 is glass, 2.42 is diamond.
    /// https://en.wikipedia.org/wiki/List_of_refractive_indices
    pub refractive_index: f64,
    /// Absorption coefficient of the material.
    /// 0.0 is transparent, 1.0 is opaque.
    /// https://en.wikipedia.org/wiki/Absorption_(electromagnetic_radiation)
    pub absorption_coefficient: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Vec3::new(255.0, 255.0, 255.0),
            albedo: 1.0,
            roughness: 0.5,
            refractive_index: 1.0,
            absorption_coefficient: 0.0,
        }
    }
}
