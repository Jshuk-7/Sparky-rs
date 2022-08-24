use super::types::*;

use gl::types::*;

use std::{mem, os::raw::*};

/// A generic buffer, can hold many types of data (everything in the enum BufferDataType)
pub struct Buffer {
    id: GLuint,
    type_: GLenum,
    usage: GLenum,
}

impl Buffer {
	/// Creates a new buffer
	/// 
	/// ## Arguments
	/// - 'ty' - The type of the buffer
	/// - 'usage' - The way the buffer will be used
	/// 
	/// ## Example
	/// ```
	/// let buffer = Buffer::new(BufferType::Vertex, BufferUsage::Multi);
	/// ```
    pub fn new(ty: BufferType, usage: BufferUsage) -> Self {
        let mut id = 0;

        unsafe { gl::GenBuffers(1, &mut id) }

        let type_: GLuint = ty.buf_type();

        let buffer_usage: GLuint = usage.buf_usage();

        Self {
            id,
            type_,
            usage: buffer_usage,
        }
    }

    /// Binds a buffer in the opengl state machine.
    /// 
    /// ## Usage
    /// A buffer must be bound before submitting data to it
    ///
    /// ## Example
    /// ```
    /// buffer.bind();
    /// //send data to gpu buffer
    /// ```
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.type_, self.id) }
    }

    /// Unbinds a buffer in the opengl state machine.
    /// 
    /// ## Usage
    /// A buffer should be unbound before binding another,</br>
    /// however it is not absolutely neccessary in performance critical scenarios
    ///
    /// ## Example
    /// ```
    /// 'game_loop: loop {
    ///     //game code...
    /// }
    ///
    /// //cleanup
    /// buffer.unbind();
    /// ```
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(self.type_, 0) }
    }

    /// Sends data to the gpu to be drawn
    /// 
	/// ## Arguments
	/// - 'data' - The data to be stored on the gpu
	/// 
    /// ## Usage
    /// A buffer must be bound before submitting any data
	/// 
    /// ## Example
    /// ```
	/// buffer.bind();
	/// let vertices = [0, 1, 0, 1];
    /// buffer.submit_data(&vertices);
	/// buffer.unbind();
    /// ```
    pub fn submit_data<T>(&self, data: &[T]) {
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
