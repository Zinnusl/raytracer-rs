use contracts::*;

use crate::interval::Interval;
use crate::material::Material;
use crate::vec3::*;

#[derive(Debug, PartialEq, Clone)]
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

/// A ray bounded by two intervals
///
pub struct UpRightBoundedRay {
    pub ray: Ray,
    pub up_vec: UnitVec3,
    pub right_vec: UnitVec3,
    pub up_interval: Interval<f64>,
    pub right_interval: Interval<f64>,
}

impl UpRightBoundedRay {
    pub fn new(
        ray: Ray,
        up_vec: UnitVec3,
        right_vec: UnitVec3,
        up_interval: Interval<f64>,
        right_interval: Interval<f64>,
    ) -> UpRightBoundedRay {
        UpRightBoundedRay {
            ray,
            up_vec,
            right_vec,
            up_interval,
            right_interval,
        }
    }

    // ensures is in bounds
    // #[ensures(
    //     ret.x >= self.up_interval.lower_bound
    //         && ret.x <= self.up_interval.upper_bound
    //         && ret.y >= self.right_interval.lower_bound
    //         && ret.y <= self.right_interval.upper_bound
    // )]
    pub fn at(&self, t: f64, u: f64, v: f64) -> Pnt3 {
        self.ray.at(t) + self.up_vec * u + self.right_vec * v
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntersectResult {
    /// The distance along the ray to the intersection point
    pub t: f64,
    /// The normal of the surface at the intersection point
    pub normal: UnitVec3,
    /// The material of the surface at the intersection point
    pub material: Material,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_up_right_bounded_ray_at() {
        let origin = Pnt3::new(0.0, 0.0, 0.0);
        let dir = UnitVec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, dir);

        let up_vec = UnitVec3::new(0.0, 1.0, 0.0);
        let right_vec = UnitVec3::new(0.0, 0.0, 1.0);

        let up_interval = Interval::new(0.0, 1.0);
        let right_interval = Interval::new(0.0, 1.0);

        let bounded_ray =
            UpRightBoundedRay::new(ray, up_vec, right_vec, up_interval, right_interval);

        let t = 0.5;
        let u = 0.25;
        let v = 0.75;

        let result = bounded_ray.at(t, u, v);

        assert_eq!(result, Pnt3::new(0.5, 0.25, 0.75));
    }

    #[test]
    fn test_up_right_bounded_ray_at_out_of_bounds() {
        let origin = Pnt3::new(0.0, 0.0, 0.0);
        let dir = UnitVec3::new(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, dir);

        let up_vec = UnitVec3::new(0.0, 1.0, 0.0);
        let right_vec = UnitVec3::new(0.0, 0.0, 1.0);

        let up_interval = Interval::new(0.0, 1.0);
        let right_interval = Interval::new(0.0, 1.0);

        let bounded_ray =
            UpRightBoundedRay::new(ray, up_vec, right_vec, up_interval, right_interval);

        let t = 0.5;
        let u = 1.5; // out of bounds
        let v = 0.75;

        // Depending on your implementation, this might panic, return an error, or clamp the value to the interval.
        // Adjust the test accordingly.
        let result = bounded_ray.at(t, u, v);

        // assert_eq!(result, ???);
    }
}
