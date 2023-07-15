extern crate gl;

use crate::gl::types::{GLuint, GLenum, GLboolean, GLsizei, GLvoid, GLint};

pub type VertexAttrib = (GLuint, GLint, GLenum, GLboolean, GLsizei, *const GLvoid);

pub struct Mesh {
    vao: GLuint, // For attrib pointers + vbo
    vbo: GLuint, // For vertices data
    ebo: GLuint, // For indices

    indices_count: GLsizei,
    index_type: GLenum,
}

impl Mesh {
    pub fn new() -> Self {
        let mut this = Mesh { 
            vao: 0, 
            vbo: 0, 
            ebo: 0, 
            indices_count: 0,
            index_type: gl::UNSIGNED_INT 
        };

        unsafe {
            gl::GenBuffers(1, &mut this.vbo);
            gl::GenBuffers(1, &mut this.ebo);
            gl::GenVertexArrays(1, &mut this.vao);
        
            this.bind();
            this.unbind();
        }

        this
    }

    pub fn bind(&self) {
        unsafe {
            // For some whatever reason VAO does not automatically bind VBO and EBO.
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0); 
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn set_vertex_data<T>(&mut self, data: &[T], usage: GLenum) {
        let size = std::mem::size_of::<T>() * data.len();
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(gl::ARRAY_BUFFER, size as gl::types::GLsizeiptr, data.as_ptr() as *const _, usage);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }

    pub fn set_indices<T>(&mut self, data: &[T], indices_count: usize, usage: GLenum, index_type: GLenum) {
        let size = std::mem::size_of::<T>() * data.len();
        self.index_type = index_type;
        self.indices_count = indices_count as i32;
        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size as gl::types::GLsizeiptr, data.as_ptr() as *const _, usage);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }

    pub fn set_indices_u32_tuples(&mut self, data: &[(u32, u32, u32)], usage: GLenum) {
        self.set_indices(data, data.len() * 3, usage, gl::UNSIGNED_INT);
    }
    
    pub fn draw(&self) {
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices_count(), self.index_type, std::ptr::null());
        }
    }

    pub fn indices_count(&self) -> GLsizei {
        self.indices_count
    }

    pub fn bind_vertex_attribs<I>(&mut self, attribs: I) 
        where I: IntoIterator<Item = VertexAttrib>
    {
        self.bind();
        unsafe {
            for (index, size, va_type, normalized, stride, pointer) in attribs {
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(index, size, va_type, normalized, stride, pointer);
            }
        }
        //self.unbind();
    }

    // [(element_count, element_size, element_type)]
    pub fn bind_consecutive_attribs(&mut self, start_id: u32, attribs: &[(usize, usize, GLenum)])
    {
        self.bind();
        let mut pointer = 0;
        let mut index = start_id;
        
        let stride: usize = attribs.iter()
            .map(|(count, size, _)| *count * *size)
            .sum();

        for (element_count, element_size, element_type) in attribs {
            unsafe {
                gl::EnableVertexAttribArray(index);
                gl::VertexAttribPointer(
                    index, 
                    *element_count as GLsizei, 
                    *element_type, 
                    gl::FALSE, 
                    stride as GLint, 
                    pointer as *const _
                );

            }

            index += 1;
            pointer += *element_count * *element_size;
        }
    }

    pub fn vao(&self) -> gl::types::GLuint {
        self.vao
    }
    
    pub fn vbo(&self) -> gl::types::GLuint {
        self.vbo
    }

    pub fn ebo(&self) -> gl::types::GLuint {
        self.ebo
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}