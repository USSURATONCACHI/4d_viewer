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
            //gl::GenBuffers(1, &mut this.ebo);
            gl::GenVertexArrays(1, &mut this.vao);

            gl::BindVertexArray(this.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, this.vbo);
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, this.ebo);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            //gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        this
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
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
            //println!("EBO: {} \t| size: {size}", self.ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, size as gl::types::GLsizeiptr, data.as_ptr() as *const _, usage);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
    
    pub fn draw(&self) {
        self.bind();
        //println!("Indices count is: {}", self.indices_count);
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.indices_count(), self.index_type, std::ptr::null());
        }
        self.unbind();
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