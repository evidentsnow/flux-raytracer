use std::ops::{Add, AddAssign, Sub, SubAssign};

// 2D Point
struct Point2 {
    x: f64,
    y: f64,
}

// 3D Point
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

// 2D Vector
struct Vec2 {
    x: f64,
    y: f64,
}

// 3D Vector
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, v: Vec3) -> Point3 {
        Point3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        }
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        }
    }
}

impl SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl Vec3 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        return Vec3 { x: x, y: y, z: z };
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

    fn normalize(vector: Vec3) -> Vec3 {
        let length = ((vector.x * vector.x) + (vector.y * vector.y) + (vector.z * vector.z)).sqrt();

        return Vec3::new(vector.x / length, vector.y / length, vector.z / length);
    }
}

struct Normal3 {
    // Surface normals are like vectors, but CANNOT be added to a point, and one CANNOT take the cross product of two normals
    x: f64,
    y: f64,
    z: f64,
}

impl Normal3 {
    fn as_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    // flip surface normal so it lies in the same hemisphere as a given vector
    fn face_forward(normal: Normal3, vector: Vec3) -> Normal3 {
        if (dot_product(normal.as_vec3(), vector) < 0.0) {
            Normal3 {
                x: -normal.x,
                y: -normal.y,
                z: -normal.z,
            }
        } else {
            return normal;
        }
    }
}

fn dot_product(u: Vec3, v: Vec3) -> f64 {
    return (u.x * v.x) + (u.y * v.y) + (u.z * v.z);
}

fn abs_dot_product(u: Vec3, v: Vec3) -> f64 {
    return dot_product(u, v).abs();
}

// Use Fused Multiply Addition (FMA) (but for subtraction) to ensure high floating point accuracy. This avoids artifacts in rendered images
fn cross_product(u: &Vec3, v: &Vec3) -> Vec3 {
    return {
        Vec3::new(
            u.y.mul_add(v.z, -(u.z * v.y)), // i component: (u.y * v.z) - (u.z * v.y)
            u.x.mul_add(v.z, -(u.z * v.x)), // j component: (u.x * v.z) - (u.z * v.x)
            u.x.mul_add(v.y, -(u.y * v.x)), // k component: (u.x * v.y) - (u.y * v.x)
        )
    };
}

/*
 * Construct a local coordinate system given only a single normalized 3D Vector.
 * Cross product of two vectors is orthogonal to both.
 * Apply this two times to get a set of three orthogonal vectors for the coordinate system.
 */
fn local_coordinate_system(normalized_vector: Vec3) -> (Vec3, Vec3, Vec3) {
    // Construct first orthogonal vector from normalized vector by:
    //
    // 1) Setting a component to zero (eliminate one dimension)
    // 2) Swapping remaining two components
    // 3) Negating one of the remaining components
    // 4) Normalizing new orthogonal vector

    let orthogonal_v1: Vec3;

    // We will zero the component with the smaller value, as utilizing smaller values leads to more numerical instability than larger values
    if normalized_vector.x.abs() > normalized_vector.y.abs() {
        let length = ((normalized_vector.x * normalized_vector.x)
            + (normalized_vector.z * normalized_vector.z))
            .sqrt();

        orthogonal_v1 = Vec3::new(
            -normalized_vector.z / length,
            0.0,
            normalized_vector.x / length,
        );
    } else {
        let length = ((normalized_vector.y * normalized_vector.y)
            + (normalized_vector.z * normalized_vector.z))
            .sqrt();

        orthogonal_v1 = Vec3::new(
            0.0,
            normalized_vector.z / length,
            -normalized_vector.y / length,
        );
    }

    // Construct second orthogonal vector by taking cross product of normalized vector and orthogonal v1
    let orthogonal_v2 = cross_product(&normalized_vector, &orthogonal_v1);

    return (normalized_vector, orthogonal_v1, orthogonal_v2);
}

// TODO: Write tests to validate that these functions actually give desired results and there's no silly mistakes
