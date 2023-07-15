use gl::types::{GLuint, GLenum, GLsizeiptr};

pub struct OpenglBuffer {
    ubo: GLuint,
    buf_type: GLenum,
}

impl OpenglBuffer {
    pub fn new(buf_type: GLenum) -> Self {
        let mut ubo = 0;
        unsafe { gl::GenBuffers(1, &mut ubo); }
        
        OpenglBuffer {
            ubo, buf_type
        }
    }

    pub fn write_data<T>(&self, data: *const T, length: usize, usage: GLenum) {
        self.bind();
        unsafe {
            gl::BufferData(
                self.buf_type, 
                length as GLsizeiptr, 
                data as *const _, 
                usage
            );
        }
        self.unbind();
    }

    pub fn bind_buffer_base(&self, slot: u32) {
        self.bind();
        unsafe {
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, slot, self.ubo);
        }
        self.unbind();
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.buf_type, self.ubo);
        }
    }

    #[allow(dead_code)]
    pub fn ubo(&self) -> GLuint {
        self.ubo
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.buf_type, 0);
        }
    }
}

impl Drop for OpenglBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.ubo);
        }
    }
}
