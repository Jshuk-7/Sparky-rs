use super::buffer_types::*;

use gl::types::*;

use std::os::raw::*;

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    pub fn new(
        index: u32,
        size: i32,
        type_: BufferDataType,
        normalized: bool,
        stride: usize,
        ptr: *const c_void,
    ) -> Self {
        let new_type = match type_ {
            BufferDataType::UByte => gl::UNSIGNED_BYTE,
            BufferDataType::Uint16 => gl::UNSIGNED_SHORT,
            BufferDataType::Uint32 => gl::UNSIGNED_INT,
            BufferDataType::Byte => gl::BYTE,
            BufferDataType::Int16 => gl::SHORT,
            BufferDataType::Int32 => gl::INT,
            BufferDataType::Float32 => gl::FLOAT,
        };

        let is_normalized: GLboolean = if normalized { gl::TRUE } else { gl::FALSE };

        unsafe {
            gl::VertexAttribPointer(index, size, new_type, is_normalized, stride as GLsizei, ptr)
        }

        Self { index }
    }

    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) }
    }

    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) }
    }
}
