use crate::ray::{self, IntersectResult};
use crate::vec3::{Pnt3, UnitVec3, Vec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub pnt: Pnt3,
    pub dir: UnitVec3,
    pub width: f64,
    pub length: f64,
}

impl Line {
    pub fn new(pnt: Pnt3, dir: UnitVec3, width: f64, length: f64) -> Line {
        Line {
            pnt,
            dir,
            width,
            length,
        }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Option<IntersectResult> {
        let mut t_min = -f64::INFINITY;
        let mut t_max = f64::INFINITY;
        for i in 0..3 {
            let inv_d = 1.0 / ray.dir[i];
            let mut t_0 = (self.pnt[i] - ray.origin[i]) * inv_d;
            let mut t_1 = (self.pnt[i] + self.dir[i] * self.length - ray.origin[i]) * inv_d;
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
            if p[i] - self.pnt[i] < 0.0001 {
                normal[i] = -1.0;
                normal[i] = 1.0;
            }
        }
        Some(IntersectResult {
            t,
            normal: normal.normalize().unwrap(),
            material: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let line = Line::new(
            Pnt3::new(0.0, 0.0, 0.0),
            UnitVec3::new(0.0, 0.0, 1.0),
            50.0,
            10000.0,
        );
        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, -1.0), UnitVec3::new(0.0, 0.0, 1.0));
        let intersectresult = line.intersect(&ray).unwrap();
        let t = intersectresult.t;
        let normal = intersectresult.normal;
        assert_eq!(t, 1.0);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, -1.0));

        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, 1.0), UnitVec3::new(0.0, 0.0, 1.0));
        let intersectresult = line.intersect(&ray).unwrap();
        let t = intersectresult.t;
        let normal = intersectresult.normal;
        assert_eq!(t, 1.0);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, 1.0));

        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, 0.0), UnitVec3::new(0.0, 0.0, 1.0));
        let intersectresult = line.intersect(&ray).unwrap();
        let t = intersectresult.t;
        let normal = intersectresult.normal;
        assert_eq!(t, 0.0);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, -1.0));

        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, 0.5), UnitVec3::new(0.0, 0.0, 1.0));
        let intersectresult = line.intersect(&ray).unwrap();
        let t = intersectresult.t;
        let normal = intersectresult.normal;
        assert_eq!(t, 0.5);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, -1.0));

        let ray = ray::Ray::new(Pnt3::new(0.5, 0.5, -0.5), UnitVec3::new(0.0, 0.0, 1.0));
        let intersectresult = line.intersect(&ray).unwrap();
        let t = intersectresult.t;
        let normal = intersectresult.normal;
        assert_eq!(t, -0.5);
        assert_eq!(normal, UnitVec3::new(0.0, 0.0, -1.0));
    }
}
