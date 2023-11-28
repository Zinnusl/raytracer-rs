use contracts::*;

use crate::ray;
use crate::vec3::{Pnt3, UnitVec3, Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Cube {
    pub p1: Pnt3,
    pub p2: Pnt3,
}

impl Cube {
    #[ensures(ret.p1 != ret.p2)]
    pub fn new(p1: Pnt3, p2: Pnt3) -> Cube {
        Cube { p1, p2 }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Option<(f64, UnitVec3)> {
        let mut t_min = -f64::INFINITY;
        let mut t_max = f64::INFINITY;
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t_0 = (self.p1[i] - ray.origin[i]) * inv_d;
            let mut t_1 = (self.p2[i] - ray.origin[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t_0, &mut t_1);
            }
            t_min = t_min.max(t_0);
            t_max = t_max.min(t_1);
            if t_max <= t_min {
                return None;
            }
        }
        let t = t_min.max(0.0).min(t_max.max(0.0));
        let p = ray.at(t);
        let mut normal = Vec3::null();
        for i in 0..3 {
            if p[i] - self.p1[i] < 0.0001 {
                normal[i] = -1.0;
            } else if p[i] - self.p2[i] > -0.0001 {
                normal[i] = 1.0;
            }
        }
        Some((t, normal.normalize().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::Vec3;
}
