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

fn dot_product(u: Vec3, v: Vec3) -> f64 {
    return (u.x * v.x) + (u.y * v.y) + (u.z * v.z);
}

fn abs_dot_product(u: Vec3, v: Vec3) -> f64 {
    return dot_product(u, v).abs();
}

// Use Fused Multiply Addition (FMA) (but for subtraction) to ensure high floating point accuracy. This avoids artifacts in rendered images
fn cross_product(u: Vec3, v: Vec3) -> Vec3 {
    return {
        Vec3::new(
            u.y.mul_add(v.z, -(u.z * v.y)), // i component: (u.y * v.z) - (u.z * v.y)
            u.x.mul_add(v.z, -(u.z * v.x)), // j component: (u.x * v.z) - (u.z * v.x)
            u.x.mul_add(v.y, -(u.y * v.x)), // k component: (u.x * v.y) - (u.y * v.x)
        )
    };
}

// TRANSFORMATIONS

fn main() {
    println!("Hello, world!");
}
