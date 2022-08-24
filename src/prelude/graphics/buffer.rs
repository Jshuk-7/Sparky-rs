use super::buffer_types::*;

use gl::types::*;

use std::{mem, os::raw::*};

pub struct Buffer {
    id: GLuint,
    type_: GLenum,
    usage: GLenum,
}

impl Buffer {
    pub fn new(ty: BufferType, usage: BufferUsage) -> Self {
        let mut id = 0;

        unsafe { gl::GenBuffers(1, &mut id) }

        let type_: GLuint = match ty {
            BufferType::Vertex => gl::ARRAY_BUFFER,
            BufferType::Index => gl::ELEMENT_ARRAY_BUFFER,
        };

        let buffer_usage: GLuint = match usage {
            BufferUsage::Default => gl::STATIC_DRAW,
            BufferUsage::Single => gl::STATIC_DRAW,
            BufferUsage::Multi => gl::DYNAMIC_DRAW,
        };

        Self { id, type_, usage: buffer_usage }
    }

    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.type_, self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.type_, 0) }
    }

    pub fn store_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                self.type_,
                (data.len() * mem::size_of::<T>()) as GLsizeiptr,
                &data[0] as *const T as *const c_void,
                self.usage,
            )
        }
    }
}
