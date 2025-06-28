use crate::geometry::Vec3;
use nalgebra::Matrix4;

struct Transform {
    matrix: Matrix4<f64>,
    matrix_inv: Option<Matrix4<f64>>,
}

impl Transform {
    // default to identity matrix
    fn new() -> Transform {
        return Transform {
            matrix: Matrix4::identity(),
            matrix_inv: Some(Matrix4::identity()),
        };
    }

    fn new_from_matrix(matrix: Matrix4<f64>) -> Transform {
        return Transform {
            matrix: matrix,
            matrix_inv: matrix.try_inverse(),
        };
    }

    fn translate(delta: Vec3) -> Transform {
        Transform {
            matrix: Matrix4::new(
                1.0, 0.0, 0.0, delta.x, 0.0, 1.0, 0.0, delta.y, 0.0, 0.0, 1.0, delta.z, 0.0, 0.0,
                0.0, 1.0,
            ),
            matrix_inv: Some(Matrix4::new(
                1.0, 0.0, 0.0, -delta.x, 0.0, 1.0, 0.0, -delta.y, 0.0, 0.0, 1.0, -delta.z, 0.0,
                0.0, 0.0, 1.0,
            )),
        }
    }

    fn scale(x: f64, y: f64, z: f64) -> Transform {
        Transform {
            matrix: Matrix4::new(
                x, 0.0, 0.0, 0.0, 0.0, y, 0.0, 0.0, 0.0, 0.0, z, 0.0, 0.0, 0.0, 0.0, 1.0,
            ),
            matrix_inv: Some(Matrix4::new(
                1.0 / x,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0 / y,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0 / z,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
            )),
        }
    }
}
