use contracts::*;

use crate::vec3::*;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Pnt3,
    pub dir: UnitVec3,
}

impl Ray {
    #[ensures(ret.origin == origin)]
    pub fn new(origin: Pnt3, dir: UnitVec3) -> Ray {
        Ray { origin, dir }
    }

    #[ensures(ret == self.origin + self.dir * t)]
    pub fn at(&self, t: f64) -> Pnt3 {
        self.origin + self.dir * t
    }
}
