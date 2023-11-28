use contracts::*;

use crate::vec3::{Pnt3, UnitVec3};

#[derive(Debug, Clone, PartialEq)]
pub struct Camera {
    pub focal_length: f64,

    pub center: Pnt3,
    pub up: UnitVec3,
    pub right: UnitVec3,
}

#[invariant(self.up.len() == 1.0)]
#[invariant(self.right.len() == 1.0)]
#[invariant(self.up.cross(self.right) == 1.0)]
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
