use std::f32::consts::PI;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

pub const TAU: f32 = 2.0 * PI;

macro_rules! define_vec {
    ($name:ident, $size:expr) => (
        /// A column vector.
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $name(pub [f32; $size]);

        impl $name {
            /// Create a vector with all fields set to zero.
            pub fn zero() -> Self {
                $name([0.0; $size])
            }

            /// Calculate the square of the length (or norm) of the vector. Slightly faster than
            /// `length`.
            pub fn length_squared(self) -> f32 {
                self.dot(self)
            }

            /// Calculate the the length (or norm) of the vector.
            pub fn length(self) -> f32 {
                self.length_squared().sqrt()
            }

            /// Normalize the vector so that it has the same orientation but a length of 1.
            pub fn normalize(&mut self) {
                let length = self.length();

                for i in 0..$size {
                    self[i] /= length;
                }
            }

            /// Calculate the vector dot product.
            pub fn dot(self, other: Self) -> f32 {
                let mut result = 0.0;

                for i in 0..$size {
                    result += self[i] * other[i];
                }

                result
            }
        }

        impl Index<usize> for $name {
            type Output = f32;

            fn index(&self, i: usize) -> &f32 {
                &self.0[i]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut f32 {
                &mut self.0[i]
            }
        }

        impl Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                let mut result = $name::zero();

                for i in 0..$size {
                    result[i] = self[i] + other[i];
                }

                result
            }
        }

        impl Sub for $name {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                let mut result = $name::zero();

                for i in 0..$size {
                    result[i] = self[i] - other[i];
                }

                result
            }
        }
    );
}

define_vec!(Vec3, 3);
define_vec!(Vec4, 4);

impl Vec3 {
    /// Calculate the vector cross product.
    pub fn cross(self, other: Self) -> Self {
        Vec3([
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0],
        ])
    }
}

/// A matrix stored in column-major order.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [[f32; 4]; 4]);

impl Mat4 {
    /// The zero matrix.
    pub fn zero() -> Self {
        Mat4([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    /// The identity matrix.
    pub fn identity() -> Self {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Build a matrix representing a scaling by the given factors.
    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Mat4([
            [x,   0.0, 0.0, 0.0],
            [0.0, y,   0.0, 0.0],
            [0.0, 0.0, z,   0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Build a matrix representing a translation.
    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [x,   y,   z,   1.0],
        ])
    }

    /// Build a matrix representing a rotation around the X-axis by the given angle (in radians).
    pub fn rotate_x(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [1.0, 0.0,  0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin,  cos, 0.0],
            [0.0, 0.0,  0.0, 1.0],
        ])
    }

    /// Build a matrix representing a rotation around the Y-axis by the given angle (in radians).
    pub fn rotate_y(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [ cos, 0.0, sin, 0.0],
            [ 0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [ 0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// Build a matrix representing a rotation around the Z-axis by the given angle (in radians).
    pub fn rotate_z(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [cos, -sin, 0.0, 0.0],
            [sin,  cos, 0.0, 0.0],
            [0.0,  0.0, 1.0, 0.0],
            [0.0,  0.0, 0.0, 1.0],
        ])
    }

    /// Build a camera view matrix with the camera at `eye` looking toward `center` with `up` as
    /// the vertical direction.
    pub fn look_at(eye: Vec3, center: Vec3, up: Vec3) -> Self {
        // Set the Z-axis to the unit vector pointing from the center toward the eye (the depth
        // axis).
        let mut z = eye - center;
        z.normalize();

        // Make the Y-axis the vertical direction.
        let mut y = up;

        // Make the X-axis perpendicular to Y and Z, pointing to the right.
        let mut x = y.cross(z);

        // Make the Y-axis perpendicular to Z and X.
        y = z.cross(x);

        // Normalize the axes to unit vectors.
        x.normalize();
        y.normalize();

        // Build the rotation/translation matrix that transforms coordinates to the new coordinate
        // system.
        Mat4([
            [ x[0],        y[0],        z[0],       0.0],
            [ x[1],        y[1],        z[1],       0.0],
            [ x[2],        y[2],        z[2],       0.0],
            [-x.dot(eye), -y.dot(eye), -z.dot(eye), 1.0],
        ])
    }

    /// Build a perspective projection matrix with the given vertical field of view (in radians),
    /// aspect ratio, and Z-axis clipping distances.
    pub fn perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> Self {
        assert!(aspect != 0.0);
        assert!(z_near != z_far);

        let f = 1.0 / (fov_y / 2.0).tan();
        let z_diff = z_near - z_far;

        let mut result = Mat4::zero();
        result[0][0] = f / aspect;
        result[1][1] = f;
        result[2][2] = (z_near + z_far) / z_diff;
        result[2][3] = -1.0;
        result[3][2] = (2.0 * z_near * z_far) / z_diff;
        result
    }
}

impl Index<usize> for Mat4 {
    type Output = [f32; 4];

    fn index(&self, col: usize) -> &[f32; 4] {
        &self.0[col]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, col: usize) -> &mut [f32; 4] {
        &mut self.0[col]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::zero();

        for col in 0..4 {
            for row in 0..4 {
                for i in 0..4 {
                    result[col][row] += self[i][row] * other[col][i];
                }
            }
        }

        result
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, vec: Vec4) -> Vec4 {
        let mut result = Vec4::zero();

        for col in 0..4 {
            for row in 0..4 {
                result[row] += self[col][row] * vec[col];
            }
        }

        result
    }
}

#[test]
fn test_math() {
    let scale = Mat4::scale(2.0, 2.0, 2.0);
    let trans = Mat4::translate(1.0, 2.0, 3.0);
    let combined = trans * scale;

    let original = Vec4([3.0, 3.0, 3.0, 1.0]);
    let expected = Vec4([7.0, 8.0, 9.0, 1.0]);

    assert_eq!(expected, combined * original);
}
