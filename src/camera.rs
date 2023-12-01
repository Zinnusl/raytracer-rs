use contracts::*;
use intervals_general::bound_pair::BoundPair;

use crate::ray::{Ray, UpRightBoundedRay};
use crate::vec3::{Pnt3, UnitVec3};
use intervals_general::interval::Interval;

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub focal_length: f64,

    pub center: Pnt3,
    pub up: UnitVec3,
    pub right: UnitVec3,
}

// #[invariant(self.up.cross(self.right) == Vec3::)]
impl Camera {
    /// Creates a new camera looking at the target from the origin.
    /// https://stackoverflow.com/questions/59942023/what-is-an-algorithm-for-look-at-function
    #[requires(origin != target)]
    pub fn look_at(origin: Pnt3, target: Pnt3) -> Camera {
        let zaxis = (target - origin).normalize().unwrap();
        let xaxis = match zaxis
            .cross(Pnt3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            })
            .normalize()
        {
            Ok(xaxis) => xaxis,
            Err(_) => zaxis
                .cross(Pnt3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                })
                .normalize()
                .unwrap(),
        };

        let yaxis = xaxis.cross(zaxis).normalize().unwrap();

        Camera {
            focal_length: (target - origin).len(),
            center: origin,
            up: yaxis,
            right: xaxis,
        }
    }

    pub fn new(focal_length: f64, center: Pnt3, up: UnitVec3, right: UnitVec3) -> Camera {
        Camera {
            focal_length,
            center,
            up,
            right,
        }
    }

    pub fn get_rays(
        &self,
        image_width: u32,
        image_height: u32,
    ) -> impl Iterator<Item = UpRightBoundedRay> + '_ {
        (0..image_width * image_height).map(move |i| {
            let x = (i % image_width) as f64;
            let y = (i / image_height) as f64;
            let dist_right = |x| (2.0 * (x + 0.5) / image_height as f64 - 1.0) * self.focal_length;
            let dist_up = |y| (2.0 * (y + 0.5) / image_width as f64 - 1.0) * self.focal_length;
            let right = self.right * dist_right(x);
            let up = self.up * dist_up(y);
            let pnt = self.center + right + up;
            let ray = Ray::new(pnt, self.up.cross(self.right).normalize().unwrap());
            UpRightBoundedRay::new(
                ray,
                up.normalize().unwrap(),
                right.normalize().unwrap(),
                Interval::Closed {
                    bound_pair: BoundPair::new(dist_right(-0.5), dist_right(0.5)).unwrap(),
                },
                Interval::Closed {
                    bound_pair: BoundPair::new(dist_up(-0.5), dist_up(0.5)).unwrap(),
                },
            )
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn camera_look_at() {
        let camera = Camera::look_at(
            Pnt3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
            Pnt3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        assert_eq!(camera.up.x, 0.0);
        assert_eq!(camera.up.y, 1.0);
        assert_eq!(camera.up.z, 0.0);
        assert_eq!(camera.right.x, 0.0);
        assert_eq!(camera.right.y, 0.0);
        assert_eq!(camera.right.z, -1.0);

        // Other way
        let camera = Camera::look_at(
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 100.0,
                y: 0.0,
                z: 0.0,
            },
        );
        assert_eq!(camera.up.x, 0.0);
        assert_eq!(camera.up.y, 1.0);
        assert_eq!(camera.up.z, 0.0);
        assert_eq!(camera.right.x, 0.0);
        assert_eq!(camera.right.y, 0.0);
        assert_eq!(camera.right.z, 1.0);

        // Looking down (0, -1, 0)
        let camera = Camera::look_at(
            Vec3 {
                x: 0.0,
                y: 100.0,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        );
        assert_eq!(camera.up.x, 0.0);
        assert_eq!(camera.up.y, 0.0);
        assert_eq!(camera.up.z, 1.0);
        assert_eq!(camera.right.x, -1.0);
        assert_eq!(camera.right.y, 0.0);
        assert_eq!(camera.right.z, 0.0);
    }
}
