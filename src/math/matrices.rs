use crate::prelude::*;

pub fn orthographic_view_matrix(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Matrix4x4  {
    let width_inverse = 1.0 / (right - left);
    let height_inverse = 1.0 / (bottom - top);
    let depth_inverse = 1.0 / (far - near);

    Matrix4x4::from([
        [2.0 * width_inverse, 0.0, 0.0, -(right + left) * width_inverse],
        [0.0, 2.0 * height_inverse, 0.0, -(bottom + top) * height_inverse],
        [0.0, 0.0, depth_inverse, -near * depth_inverse],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn translation_matrix(to_translate: Vector) -> Matrix4x4 {
    Matrix4x4::from([
        [1.0, 0.0, 0.0, to_translate.x],
        [0.0, 1.0, 0.0, to_translate.y],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ])
}

pub fn scale_matrix(dimensions: Dimensions) -> Matrix4x4  {
    Matrix4x4::from([
        [dimensions.width, 0.0, 0.0, 0.0],
        [0.0, dimensions.height, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ])
}

#[derive(Copy, Clone, PartialEq)]
pub struct Matrix4x4 {
    m11:f32,
    m12:f32,
    m13:f32,
    m14:f32,
    m21:f32,
    m22:f32,
    m23:f32,
    m24:f32,
    m31:f32,
    m32:f32,
    m33:f32,
    m34:f32,
    m41:f32,
    m42:f32,
    m43:f32,
    m44:f32
}

impl Matrix4x4 {
    pub fn into_column_major(self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m21, self.m31, self.m41],  
            [self.m12, self.m22, self.m32, self.m42],  
            [self.m13, self.m23, self.m33, self.m43],  
            [self.m14, self.m24, self.m34, self.m44],  
        ]
    }
}

impl From<[[f32; 4]; 4]> for Matrix4x4 {
    fn from(data: [[f32; 4]; 4]) -> Self {
        Self {
            m11:data[0][0],
            m12:data[0][1],
            m13:data[0][2],
            m14:data[0][3],
            m21:data[1][0],
            m22:data[1][1],
            m23:data[1][2],
            m24:data[1][3],
            m31:data[2][0],
            m32:data[2][1],
            m33:data[2][2],
            m34:data[2][3],
            m41:data[3][0],
            m42:data[3][1],
            m43:data[3][2],
            m44:data[3][3] 
        }
    }
}

impl Into<[[f32; 4]; 4]> for Matrix4x4 {
    fn into(self) -> [[f32; 4]; 4] {
        [
            [self.m11, self.m12, self.m13, self.m14],  
            [self.m21, self.m22, self.m23, self.m24],  
            [self.m31, self.m32, self.m33, self.m34],  
            [self.m41, self.m42, self.m43, self.m44],  
        ]
    }
}

impl Mul for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::from([
            [
                (self.m11 * rhs.m11) + (self.m12 * rhs.m21) + (self.m13 * rhs.m31) + (self.m14 * rhs.m41), 
                (self.m11 * rhs.m12) + (self.m12 * rhs.m22) + (self.m13 * rhs.m32) + (self.m14 * rhs.m42),
                (self.m11 * rhs.m13) + (self.m12 * rhs.m23) + (self.m13 * rhs.m33) + (self.m14 * rhs.m43),
                (self.m11 * rhs.m14) + (self.m12 * rhs.m24) + (self.m13 * rhs.m34) + (self.m14 * rhs.m44)
            ],
            [
                (self.m21 * rhs.m11) + (self.m22 * rhs.m21) + (self.m23 * rhs.m31) + (self.m24 * rhs.m41), 
                (self.m21 * rhs.m12) + (self.m22 * rhs.m22) + (self.m23 * rhs.m32) + (self.m24 * rhs.m42),
                (self.m21 * rhs.m13) + (self.m22 * rhs.m23) + (self.m23 * rhs.m33) + (self.m24 * rhs.m43),
                (self.m21 * rhs.m14) + (self.m22 * rhs.m24) + (self.m23 * rhs.m34) + (self.m24 * rhs.m44)
            ],
            [
                (self.m31 * rhs.m11) + (self.m32 * rhs.m21) + (self.m33 * rhs.m31) + (self.m34 * rhs.m41), 
                (self.m31 * rhs.m12) + (self.m32 * rhs.m22) + (self.m33 * rhs.m32) + (self.m34 * rhs.m42),
                (self.m31 * rhs.m13) + (self.m32 * rhs.m23) + (self.m33 * rhs.m33) + (self.m34 * rhs.m43),
                (self.m31 * rhs.m14) + (self.m32 * rhs.m24) + (self.m33 * rhs.m34) + (self.m34 * rhs.m44)
            ],
            [
                (self.m41 * rhs.m11) + (self.m42 * rhs.m21) + (self.m43 * rhs.m31) + (self.m44 * rhs.m41), 
                (self.m41 * rhs.m12) + (self.m42 * rhs.m22) + (self.m43 * rhs.m32) + (self.m44 * rhs.m42),
                (self.m41 * rhs.m13) + (self.m42 * rhs.m23) + (self.m43 * rhs.m33) + (self.m44 * rhs.m43),
                (self.m41 * rhs.m14) + (self.m42 * rhs.m24) + (self.m43 * rhs.m34) + (self.m44 * rhs.m44)
            ]
        ])
    }
}