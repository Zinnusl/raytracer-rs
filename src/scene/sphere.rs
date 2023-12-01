use contracts::*;

use crate::ray;
use crate::vec3::{Pnt3, UnitVec3};

#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    pub mid: Pnt3,
    pub r: f64,
}

impl Sphere {
    #[requires(r > 0.0)]
    #[ensures(ret.mid == mid)]
    #[ensures(ret.r == r)]
    pub fn new(mid: Pnt3, r: f64) -> Sphere {
        Sphere { mid, r }
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Option<(f64, UnitVec3)> {
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * ray.dir.dot(ray.origin - self.mid);
        let c = (ray.origin - self.mid).dot(ray.origin - self.mid) - self.r * self.r;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t_1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let t_2 = (-b - discriminant.sqrt()) / (2.0 * a);
        if t_1 < 0.0 && t_2 < 0.0 {
            return None;
        }
        let t = t_1.max(0.0).min(t_2.max(0.0));
        Some((t, (ray.at(t) - self.mid).normalize().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn sphere_new() {
        let sphere = Sphere {
            mid: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            r: 4.0,
        };
        assert_eq!(sphere.mid.x, 1.0);
        assert_eq!(sphere.mid.y, 2.0);
        assert_eq!(sphere.mid.z, 3.0);
        assert_eq!(sphere.r, 4.0);
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(
            Pnt3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            UnitVec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        );
        let point = ray.at(2.0);
        assert_eq!(point.x, 3.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn ray_sphere_intersection() {
        let sphere = Sphere {
            mid: Pnt3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            r: 1.0,
        };
        let ray = Ray::new(
            Pnt3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            UnitVec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        );
        let intersection = sphere.intersect(&ray);
        assert_eq!(
            intersection,
            Some((
                1.0,
                UnitVec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0
                }
            ))
        );

        let ray = Ray::new(
            Pnt3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            UnitVec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        );
        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection, None);

        let ray = Ray::new(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: -2.0,
            },
            UnitVec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );

        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection, None);

        let sphere = Sphere {
            mid: Vec3 {
                x: 454.0,
                y: 303.0,
                z: -50.0,
            },
            r: 1.0,
        };
        let ray = Ray::new(
            Vec3 {
                x: 455.0,
                y: 255.0,
                z: 20.0,
            },
            UnitVec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            },
        );

        let intersection = sphere.intersect(&ray);
        assert_eq!(intersection, None);
        let sphere = Sphere {
            mid: Pnt3 {
                x: 454.0,
                y: 303.0,
                z: -500.0,
            },
            r: 100.0,
        };
        let intersection = sphere.intersect(&ray);
        assert_eq!(
            intersection,
            Some((
                432.2788508967193,
                UnitVec3 {
                    x: 0.009999999999999995,
                    y: -0.4799999999999998,
                    z: 0.8772114910328067
                }
            ))
        );
    }
}
