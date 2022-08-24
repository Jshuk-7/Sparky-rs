use gl::types::*;

/// Vertex Array Object, an opengl state object that holds the currently bound buffers and shaders
pub struct Vao {
    id: GLuint,
}

impl Vao {
    /// Creates a new Vertex Array Object
    ///
    /// ## Example
    /// ```
    /// let vao = Vao::new();
    /// ```
    pub fn new() -> Self {
        let mut id = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) }

        Self { id }
    }

    /// Binds a vao in the opengl state machine.
    /// 
    /// ## Usage
    /// A vao must be bound before submiting data to a buffer, or making a draw call
    ///
    /// ## Example
    /// ```
    /// vao.bind();
    /// //send data to gpu buffer
    /// ```
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    /// Unbinds a vao in the opengl state machine.
    /// 
    /// ## Usage
    /// A vao should be unbound before binding another,</br>
    /// however it is not absolutely neccessary in performance critical scenarios
    ///
    /// ## Example
    /// ```
    /// 'game_loop: loop {
    ///     //game code...
    /// }
    ///
    /// //cleanup
    /// vao.unbind();
    /// ```
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
