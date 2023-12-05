use random::Source;

use crate::camera::Camera;
use crate::ray::{Ray, UpRightBoundedRay};
use crate::vec3::Vec3;
use intervals_general::bound_pair::BoundPair;
use intervals_general::interval::Interval;

/// A cluster of rays that are close to each other.
pub struct SampleCluster {
    pub ray: UpRightBoundedRay,
    pub camera: Camera,
    rand: random::Default,
    current_iteration: usize,
}

impl SampleCluster {
    const CLUSTER_SIZE: usize = 256;
    /// Creates a new sample cluster from a camera and a ray.
    /// Returns an iterator over the rays in the cluster.
    pub fn from_camera_ray(
        camera: Camera,
        ray: UpRightBoundedRay,
    ) -> impl Iterator<Item = Ray> + (ExactSizeIterator) + Send {
        let cluster = SampleCluster {
            rand: random::default(ray.ray.origin.x as u64),
            ray,
            camera,
            current_iteration: 0,
        };
        cluster
    }

    /// Samples from the interval with the given random number generator.
    /// Returns a value in the interval.
    fn sample_from(interval: Interval<f64>, rand: &mut impl random::Source) -> f64 {
        match interval {
            Interval::Closed { bound_pair } => {
                let lower = bound_pair.left();
                let upper = bound_pair.right();
                let rand_num = rand.read::<f64>();
                lower + (upper - lower) * rand_num
            }
            _ => unimplemented!(),
        }
    }
}

// Impl Iterator instead of returning an iterator
impl Iterator for SampleCluster {
    type Item = Ray;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (Self::CLUSTER_SIZE, Some(Self::CLUSTER_SIZE))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_iteration >= Self::CLUSTER_SIZE {
            return None;
        }
        self.current_iteration += 1;

        // Some(self.ray.ray.clone())

        let x = SampleCluster::sample_from(self.ray.right_interval, &mut self.rand);
        let y = SampleCluster::sample_from(self.ray.up_interval, &mut self.rand);
        // println!(
        //     "x: {}, y: {}, interval: {:?}",
        //     x, y, self.ray.up_interval
        // );

        let right = self.camera.right * x;
        let up = self.camera.up * y;
        let pnt = self.ray.ray.origin + right + up;
        let ray = Ray::new(pnt, self.ray.ray.dir);

        Some(ray)
    }
}

impl ExactSizeIterator for SampleCluster {
    fn len(&self) -> usize {
        Self::CLUSTER_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSource {
        pub rand: u64,
    }

    impl Source for MockSource {
        fn read_u64(&mut self) -> u64 {
            let tmp = self.rand;
            if self.rand == std::u64::MAX {
                self.rand = 0;
            } else {
                self.rand += 1;
            }
            tmp
        }
    }

    #[test]
    fn sample_from() {
        let mut source = MockSource { rand: 0 };
        let interval = Interval::Closed {
            bound_pair: BoundPair::new(-1.0, 1.0).unwrap(),
        };
        let result = SampleCluster::sample_from(interval, &mut source);
        assert_eq!(result, -1.0);

        let mut source = MockSource {
            rand: std::u64::MAX,
        };
        let result = SampleCluster::sample_from(interval, &mut source);
        assert_eq!(result, 1.0);

        let mut source = MockSource {
            rand: std::u64::MAX / 2,
        };
        let result = SampleCluster::sample_from(interval, &mut source);
        assert_eq!(result, 0.0);
    }
}

