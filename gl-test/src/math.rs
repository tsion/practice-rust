use gl::types::*;
use std::f32::consts::PI;
use std::ops::{Index, IndexMut, Mul};

pub const TAU: GLfloat = 2.0 * PI;

macro_rules! define_vec {
    ($name:ident, $size:expr) => (
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct $name(pub [GLfloat; $size]);

        impl $name {
            pub fn zero() -> Self {
                $name([0.0; $size])
            }

            /// Calculate the square of the length (or norm) of the vector. Slightly faster than
            /// `length`.
            pub fn length_squared(&self) -> GLfloat {
                let mut sum = 0.0;

                for i in 0..$size {
                    sum += self[i] * self[i];
                }

                sum
            }

            /// Calculate the the length (or norm) of the vector.
            pub fn length(&self) -> GLfloat {
                self.length_squared().sqrt()
            }

            /// Normalize the vector so that it has the same orientation but a length of 1.
            pub fn normalize(&mut self) {
                let length = self.length();

                for i in 0..$size {
                    self[i] /= length;
                }
            }
        }

        impl Index<usize> for $name {
            type Output = GLfloat;

            fn index(&self, i: usize) -> &GLfloat {
                &self.0[i]
            }
        }

        impl IndexMut<usize> for $name {
            fn index_mut(&mut self, i: usize) -> &mut GLfloat {
                &mut self.0[i]
            }
        }
    );
}

define_vec!(Vec3, 3);
define_vec!(Vec4, 4);

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [[GLfloat; 4]; 4]);

impl Mat4 {
    pub fn zero() -> Self {
        Mat4([
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ])
    }

    pub fn identity() -> Self {
        Mat4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn scale(x: GLfloat, y: GLfloat, z: GLfloat) -> Self {
        Mat4([
            [x,   0.0, 0.0, 0.0],
            [0.0, y,   0.0, 0.0],
            [0.0, 0.0, z,   0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn translate(x: GLfloat, y: GLfloat, z: GLfloat) -> Self {
        Mat4([
            [1.0, 0.0, 0.0, x  ],
            [0.0, 1.0, 0.0, y  ],
            [0.0, 0.0, 1.0, z  ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix representing a rotation around the X-axis by the given angle (in radians).
    pub fn rotate_x(angle: GLfloat) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [1.0, 0.0,  0.0, 0.0],
            [0.0, cos, -sin, 0.0],
            [0.0, sin,  cos, 0.0],
            [0.0, 0.0,  0.0, 1.0],
        ])
    }

    /// A matrix representing a rotation around the Y-axis by the given angle (in radians).
    pub fn rotate_y(angle: GLfloat) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [ cos, 0.0, sin, 0.0],
            [ 0.0, 1.0, 0.0, 0.0],
            [-sin, 0.0, cos, 0.0],
            [ 0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// A matrix representing a rotation around the Z-axis by the given angle (in radians).
    pub fn rotate_z(angle: GLfloat) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();

        Mat4([
            [cos, -sin, 0.0, 0.0],
            [sin,  cos, 0.0, 0.0],
            [0.0,  0.0, 1.0, 0.0],
            [0.0,  0.0, 0.0, 1.0],
        ])
    }
}

impl Index<usize> for Mat4 {
    type Output = [GLfloat; 4];

    fn index(&self, i: usize) -> &[GLfloat; 4] {
        &self.0[i]
    }
}

impl IndexMut<usize> for Mat4 {
    fn index_mut(&mut self, i: usize) -> &mut [GLfloat; 4] {
        &mut self.0[i]
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut result = Mat4::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    result[i][j] += self[i][k] * other[k][j];
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

        for i in 0..4 {
            for j in 0..4 {
                result[i] += self[i][j] * vec[j];
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