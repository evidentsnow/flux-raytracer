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
struct Vec3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vec3<T>
where
    T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Clone,
{
    fn new(x: T, y: T, z: T) -> Self {
        return Vec3 { x: x, y: y, z: z };
    }

    fn add_vectors(&self, vector: Vec3<T>) -> Vec3<T> {
        return Vec3::new(self.x + vector.x, self.y + vector.y, self.z + vector.z);
    }

    fn subtract_vectors(&self, vector: &Vec3<T>) -> Vec3<T> {
        return Vec3::new(self.x - vector.x, self.y - vector.y, self.z - vector.z);
    }

    fn scalar_multiplication(mut self, scalar: T) -> Self {
        self.x = self.x * scalar;
        self.y = self.y * scalar;
        self.z = self.z * scalar;
        return self;
    }

    fn absolute_value_vector(self) -> Self {
        return Vec3 {
            x: f64::abs(self.x),
            y: f64::abs(self.y),
            z: f64::abs(self.z),
        };
    }
}

fn dot_product(v1: Vec<T>, v2: Vec<T>) -> T {
    return (v1.x * v2.x) + (v1 * v2.y) + (v1 * v2.z);
}

fn abs_dot_product(v1: Vec<T>, v2: Vec<T>) -> T {
    return f64::abs((v1.x * v2.x) + (v1 * v2.y) + (v1 * v2.z));
}

fn cross_product() -> Vec<T> {}

// TRANSFORMATIONS

fn main() {
    println!("Hello, world!");
}
