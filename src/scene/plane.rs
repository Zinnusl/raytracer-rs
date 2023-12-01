use contracts::*;

use crate::ray;
use crate::vec3::{Pnt3, UnitVec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    pub pnt: Pnt3,
    pub normal: UnitVec3,
}

impl Plane {
    pub fn new(pnt: Pnt3, normal: UnitVec3) -> Self {
        Plane { pnt, normal }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Option<(f64, UnitVec3)> {
        let denom = self.normal.dot(ray.dir);
        if denom.abs() > 0.0001 {
            let t = (self.pnt - ray.origin).dot(self.normal) / denom;
            if t >= 0.0 {
                return Some((t, self.normal));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let plane = Plane::new(Pnt3::new(0.0, 0.0, 0.0), UnitVec3::new(0.0, 0.0, 1.0));
        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, -1.0), UnitVec3::new(0.0, 0.0, 1.0));
        let (t, normal) = plane.intersect(&ray).unwrap();
        assert_eq!(t, 1.0);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, 1.0));

        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, 1.0), UnitVec3::new(0.0, 0.0, 1.0));
        assert!(plane.intersect(&ray).is_none());
    }
}
