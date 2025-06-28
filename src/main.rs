use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

// GEOMETRY

// 2D Vector
struct Vec2 {
    x: f64,
    y: f64,
}

// 3D Vector
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3 { x: x, y: y, z: z };
    }

    fn add_vectors(&self, vector: Vec3) -> Vec3 {
        return Vec3::new(self.x + vector.x, self.y + vector.y, self.z + vector.z);
    }

    fn subtract_vectors(&self, vector: &Vec3) -> Vec3 {
        return Vec3::new(self.x - vector.x, self.y - vector.y, self.z - vector.z);
    }

    fn scalar_multiplication(mut self, scalar: f64) -> Self {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        return self;
    }

    fn absolute_value_vector(self) -> Self {
        return Vec3 {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        };
    }
}

fn dot_product(v1: Vec3, v2: Vec3) -> f64 {
    return (v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z);
}

fn abs_dot_product(v1: Vec3, v2: Vec3) -> f64 {
    return ((v1.x * v2.x) + (v1.y * v2.y) + (v1.z * v2.z)).abs();
}

fn cross_product() -> Vec3 {}

// TRANSFORMATIONS

fn main() {
    println!("Hello, world!");
}
