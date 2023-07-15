extern crate gl;

use gl::types::{GLuint, GLsizeiptr};

use crate::transform4d::{self, Mat5};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Cube = 0,
    Sphere = 1
}

use ObjectType::*;
#[repr(C)]
pub struct Object {
    pub obj_type: ObjectType,
    pub transform: Mat5,
}

pub struct GpuObjectsHandle {
    ssbo: GLuint
}

impl GpuObjectsHandle {
    pub fn new() -> Self {
        let mut ssbo = 0;

        unsafe { gl::GenBuffers(1, &mut ssbo); }
        
        GpuObjectsHandle { 
            ssbo
        }
    }

    pub fn write_objects(&mut self, objects: &[Object]) {
        let data_to_write: Box<[(ObjectType, [f32; 25])]> = objects.iter()
            .map(|obj| (
                obj.obj_type, 
                transform4d::matrix_to_array(obj.transform.try_inverse().unwrap())
            ))
            .collect();

        self.bind();
        unsafe {
            let size = data_to_write.len() * std::mem::size_of::<(ObjectType, [f32; 25])>();
            gl::BufferData(gl::SHADER_STORAGE_BUFFER, size as GLsizeiptr, data_to_write.as_ptr() as *const _, gl::STATIC_READ);
        }
        self.unbind();
    }

    pub fn bind_buffer_base(&self, slot: u32) {
        self.bind();
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, slot, self.ssbo);
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, self.ssbo);
        }
    }

    pub fn ubo(&self) -> GLuint {
        self.ssbo
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::SHADER_STORAGE_BUFFER, 0);
        }
    }
}

impl Drop for GpuObjectsHandle {
    fn drop(&mut self) {
        println!("Drop!");
        unsafe {
            gl::DeleteBuffers(1, &self.ssbo);
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