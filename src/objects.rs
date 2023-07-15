extern crate gl;

use gl::types::{GLuint, GLsizeiptr};

use crate::transform4d::{self, Mat5};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Cube = 0,
    Sphere = 1
}

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
        #[allow(dead_code)]
        struct BufObject {
            obj_type: ObjectType,
            transform: [f32; 25],
            inverse: [f32; 25],
        }

        let data_to_write: Box<[BufObject]> = objects.iter()
            .map(|obj| BufObject {
                obj_type:   obj.obj_type,
                transform:  transform4d::matrix_to_array(obj.transform.clone()),
                inverse:    transform4d::matrix_to_array(obj.transform.try_inverse().unwrap())
            })
            .collect();

        self.bind();
        unsafe {
            let size = data_to_write.len() * std::mem::size_of::<BufObject>();
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

    #[allow(dead_code)]
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
        unsafe {
            gl::DeleteBuffers(1, &self.ssbo);
        }
    }
}
