use super::types::*;

use gl::types::*;

use std::os::raw::*;

pub type CVoidPtr = *const std::ffi::c_void;

/// A description of a vertex buffer attribute.</br>
///
/// ## Usage
/// One Vertex Attribute must be created and enabled per attribute of a vertex buffer
pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    /// Creates a new Vertex Attribute
    ///
    /// ## Arguments
    /// - 'index' - The 'location' the data is in the vertex buffer
    /// - 'size' - The number of components per attribute
    /// - 'ty' - The data type of each component in the array
    /// - 'normalized' - Specifies whether fixed-point data values should be normalized (true)</br>
    ///   or converted directly as fixed-point values (false)
    /// - 'byte_stride' - The byte offset between each vertex attribute, if *stride* is 0 the vertex buffer</br>
    ///   is understood to be tightly packed
    /// - 'ptr' - The offset in the vertex buffer of the first component of the attribute
    ///
    /// ## Example
    /// ```
    /// let attribute = VertexAttribute::new(
    ///     0,
    ///     3,
    ///     BufferDataType::Float32,
    ///     false,
    ///     std::mem::size_of::<Vec3>,
    ///     std::ptr::null()
    /// );
    /// ```
    pub fn new(
        index: u32,
        size: usize,
        ty: BufferDataType,
        normalized: bool,
        byte_stride: usize,
        ptr: *const c_void,
    ) -> Self {
        let type_ = ty.data_type();

        let is_normalized: GLboolean = if normalized { gl::TRUE } else { gl::FALSE };

        let stride_ = byte_stride as GLsizei;

        unsafe { gl::VertexAttribPointer(index, size as i32, type_, is_normalized, stride_, ptr) }

        Self { index }
    }

    /// Enables an attribute in the currently bound vao.
    ///
    /// ## Usage
    /// All vertex attributes must be enabled before making a draw call
    ///
    /// ## Example
    /// ```
    /// position_attribute.enable();
    /// //send data to gpu buffer
    /// ```
    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) }
    }

    /// Disables an attribute in the currently bound vao.
    ///
    /// ## Usage
    /// A vertex attribute should be disabled when not in use,</br>
    /// however it is not absolutely neccessary in performance critical scenarios
    ///
    /// ## Example
    /// ```
    /// position_attribute.disable();
    /// //send data to gpu buffer
    /// ```
    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) }
    }
}
