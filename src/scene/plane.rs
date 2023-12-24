use crate::material::Material;
use crate::ray::*;
use crate::vec3::{Pnt3, UnitVec3, Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub pnt: Pnt3,
    pub normal: UnitVec3,
}

impl Plane {
    pub fn new(pnt: Pnt3, normal: UnitVec3) -> Self {
        Plane { pnt, normal }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<IntersectResult> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() < 1e-6 {
            return None;
        }
        let t = (self.pnt - ray.origin).dot(self.normal) / denom;
        if t < 0.0 {
            return None;
        }
        let material = Material {
            color: Vec3::new(5.0, 150.0, 5.0),
            albedo: 0.8,
            roughness: 0.8,
            refractive_index: 1.0,
            absorption_coefficient: 0.0,
        };
        Some(IntersectResult {
            t,
            normal: self.normal,
            material,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let plane = Plane::new(Pnt3::new(0.0, 0.0, 0.0), UnitVec3::new(0.0, 0.0, 1.0));
        let ray = Ray::new(Pnt3::new(0.5, 0.5, -1.0), UnitVec3::new(0.0, 0.0, 1.0));
        let intersection = plane.intersect(&ray).unwrap();
        let t = intersection.t;
        let normal = intersection.normal;
        let _mateiral = intersection.material;
        assert_eq!(t, 1.0);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, 1.0));

        let ray = Ray::new(Pnt3::new(0.5, 0.5, 1.0), UnitVec3::new(0.0, 0.0, 1.0));
        assert!(plane.intersect(&ray).is_none());
    }
}
