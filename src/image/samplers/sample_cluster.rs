use random::Source;

use crate::camera::Camera;
use crate::ray::{Ray, UpRightBoundedRay};
use crate::vec3::Vec3;
use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

/// A cluster of rays that are close to each other.
pub struct SampleCluster {
    pub original_ray: Ray,
    pub camera: Camera,
    rand: random::Default,
    current_iteration: usize,
}

impl SampleCluster {
    const CLUSTER_SIZE: usize = 1;
    /// Creates a new sample cluster from a camera and a ray.
    /// Returns an iterator over the rays in the cluster.
    pub fn from_camera_ray(camera: Camera, ray: UpRightBoundedRay) -> impl Iterator<Item = Ray> {
        let cluster = SampleCluster {
            original_ray: ray,
            camera,
            rand: random::default((1000.0f64).max(std::u64::MAX as f64) as u64),
            current_iteration: 0,
        };
        cluster
    }
}

// Impl Iterator instead of returning an iterator
impl Iterator for SampleCluster {
    type Item = UpRightBoundedRay;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration >= Self::CLUSTER_SIZE {
            return None;
        }
        self.current_iteration += 1;

        // Some(self.original_ray.clone())

        let x = (self.rand.read_f64() * 2.0) - 1.0;
        let y = (self.rand.read_f64() * 2.0) - 1.0;

        let right = self.camera.right * x;
        let up = self.camera.up * y;
        let pnt =
            self.camera.center + right * self.camera.focal_length + up * self.camera.focal_length;
        let ray = Ray::new(
            pnt,
            self.camera.up.cross(self.camera.right).normalize().unwrap(),
        );
        let ray = UpRightBoundedRay::new(
            ray,
            up.normalize().ok()?,
            right.normalize().ok()?,
            Interval::Closed {
                bound_pair: BoundPair::new(-0.5, 0.5)?,
            },
            Interval::Closed {
                bound_pair: BoundPair::new(-0.5, 0.5)?,
            },
        );

        Some(ray)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camera::Camera;
    use crate::ray::Ray;
    use crate::vec3::Vec3;

    #[test]
    fn sample_cluster() {}
}

