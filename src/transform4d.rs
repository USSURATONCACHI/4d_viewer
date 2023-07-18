#![allow(dead_code)]

extern crate nalgebra;

use nalgebra::matrix;

pub type Mat5 = nalgebra::base::Matrix5<f64>;
pub type Vec6 = nalgebra::base::Vector6<f64>;
pub type Vec5 = nalgebra::base::Vector5<f64>;
pub type Vec4 = nalgebra::base::Vector4<f64>;

/// Makes perspective matrix (perspective for -Z axis, since opengl "camera" looks in -Z direction)
pub fn perspective_matrix(fov: f64, aspect_ratio: f64, near: f64, far: f64) -> Mat5 {
    nalgebra::matrix![
        1.0 / aspect_ratio / (fov / 2.0).tan(), 0.0,                        0.0,                                0.0,    0.0;
        0.0,                                    1.0 / (fov / 2.0).tan(),    0.0,                                0.0,    0.0;
        0.0,                                    0.0,                       -(far + near) / (far - near),        0.0,   -2.0 * far * near / (far - near);
        0.0,                                    0.0,                        0.0,                                1.0,    0.0;
        0.0,                                    0.0,                       -1.0,                                0.0,    0.0;
    ]
}

pub fn scale(scale: Vec4) -> Mat5 {
    matrix![
        scale[0],  0.0,  0.0,  0.0,  0.0;
        0.0,  scale[1],  0.0,  0.0,  0.0;
        0.0,  0.0,  scale[2],  0.0,  0.0;
        0.0,  0.0,  0.0,  scale[3],  0.0;
        0.0,  0.0,  0.0,  0.0,  1.0;
    ]
}

pub fn shift(shift: Vec4) -> Mat5 {
    matrix![
        1.0,  0.0,  0.0,  0.0,  shift[0];
        0.0,  1.0,  0.0,  0.0,  shift[1];
        0.0,  0.0,  1.0,  0.0,  shift[2];
        0.0,  0.0,  0.0,  1.0,  shift[3];
        0.0,  0.0,  0.0,  0.0,  1.0;
    ]
}

pub fn scale_shift(scale: Vec4, shift: Vec4) -> Mat5 {
    matrix![
        scale[0],  0.0,  0.0,  0.0,  shift[0];
        0.0,  scale[1],  0.0,  0.0,  shift[1];
        0.0,  0.0,  scale[2],  0.0,  shift[2];
        0.0,  0.0,  0.0,  scale[3],  shift[3];
        0.0,  0.0,  0.0,  0.0,  1.0;
    ]
}

pub fn rotation_single(angle: f64, axis_from: usize, axis_into: usize) -> Mat5 {
    //  angle.cos(), -angle.sin()
    //  angle.sin(), angle.cos()
    let mut result = Mat5::identity();

    result[(axis_from, axis_from)] = angle.cos();
    result[(axis_from, axis_into)] = -angle.sin();
    result[(axis_into, axis_from)] = angle.sin();
    result[(axis_into, axis_into)] = angle.cos();

    result
}

/// Constucts matrix of 4D rotation in 6 planes
/// 
/// 4 axes: x, y, z, w
/// 
/// 6 rotation planes: xy, xz, xw, yz, yw, zw (in order)  
pub fn rotation_full(angles: Vec6) -> Mat5 {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    const W: usize = 3;

    rotation_single(angles[5], Z, W) * 
    rotation_single(angles[4], Y, W) * 
    rotation_single(angles[3], Y, Z) * 
    rotation_single(angles[2], X, W) * 
    rotation_single(angles[1], X, Z) *
    rotation_single(angles[0], X, Y)
}

/// Constucts matrix of 4D rotation in 6 planes (opposite order)
/// 
/// 4 axes: x, y, z, w
/// 
/// 6 rotation planes: zw, yw, yz, xw, xz, xy
pub fn rotation_full_inv(angles: Vec6) -> Mat5 {
    const X: usize = 0;
    const Y: usize = 1;
    const Z: usize = 2;
    const W: usize = 3;
    rotation_single(angles[0], X, Y) *
    rotation_single(angles[1], X, Z) *
    rotation_single(angles[2], X, W) * 
    rotation_single(angles[3], Y, Z) * 
    rotation_single(angles[4], Y, W) * 
    rotation_single(angles[5], Z, W) 
}

pub fn full_transform(pos: Vec4, size: Vec4, rotation: Vec6) -> Mat5 {
    shift(pos) * 
    rotation_full(rotation) *
    scale(size)
}

pub fn matrix_to_array(mat: &Mat5) -> [f32; 5*5] {
    std::array::from_fn(|i| {
        let row = i / 5;
        let col = i % 5;

        mat[(row, col)] as f32
    })
}