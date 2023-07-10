extern crate gl;

use gl::types::GLuint;

use crate::transform4d::{self, Mat5};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Cube = 0,
    Sphere = 1
}

use ObjectType::*;
pub struct Object {
    obj_type: ObjectType,
    transform: Mat5,
}

pub struct ObjectsManager {
    objects_ubo: GLuint,
}

impl ObjectsManager {
    pub fn new() -> Self {
        let mut ubo = u32::MAX;

        println!("Trying to gen ubo!");
        unsafe { gl::GenBuffers(1, &mut ubo); }
        println!("Got ubo: {ubo}");
        
        ObjectsManager { 
            objects_ubo: ubo
        }
    }

    pub fn write_objects(&mut self, objects: &[Object]) {
        let data_to_write: Box<[_]> = objects.iter()
            .map(|obj| (
                obj.obj_type, 
                obj.transform.try_inverse().unwrap()
            ))
            .map(|(obj_type, matrix)| (
                obj_type, 
                transform4d::matrix_to_array(matrix)
            ))
            .collect();
    }  
}

impl Drop for ObjectsManager {
    fn drop(&mut self) {
        println!("Drop!");
        unsafe {
            gl::DeleteBuffers(1, &self.objects_ubo);
        }
    }
}

fn default_objects() -> Vec<Object> {
    vec![
        Object {
            obj_type: Cube,
            transform: transform4d::full_transform(
                nalgebra::vector![0.0, 0.0, 0.0, 0.0],
                nalgebra::vector![2.0, 1.0, 1.0, 1.0],
                nalgebra::vector![30.0 * std::f64::consts::PI / 180.0, 0.0, 0.0, 0.0, 0.0, 0.0]
            )
        },

        Object {
            obj_type: Sphere,
            transform: transform4d::shift(nalgebra::vector![0.0, 0.0, 1.0, 0.0])
        }
    ]
}