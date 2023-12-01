use contracts::*;

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

use intervals_general::interval::Interval;

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
