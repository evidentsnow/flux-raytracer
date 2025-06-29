use crate::geometry::{Point3, Vec3};

// Rays are simply lines from vector calculus. They can be written as a vector function
// P(t) = A + tb
// P is a 3D position along a line in 3D
// A is ray origin (Point)
// t is a parameter, which affects the length of the ray
// b is a vector representing the ray direction.

struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    // Get endpoint of ray
    fn at(&self, t: f64) -> Point3 {
        return Point3 {
            x: self.origin.x + (t * self.direction.x),
            y: self.origin.y + (t * self.direction.y),
            z: self.origin.z + (t * self.direction.z),
        };
    }
}
