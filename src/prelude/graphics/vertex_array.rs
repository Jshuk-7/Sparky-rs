use gl::types::*;

pub struct Vao {
    id: GLuint,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;

        unsafe { gl::GenVertexArrays(1, &mut id) }

        Self { id }
    }

    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
}
