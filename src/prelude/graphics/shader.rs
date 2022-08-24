use super::{super::logger, types::ShaderType};

use gl::types::*;

use std::{ffi::CString, fs, ptr, str};

/// A program that gets run on the gpu to describe how to draw primitives
pub struct Shader {
    id: GLuint,
}

impl Shader {
    /// Create a new shader
    ///
    /// ## Arguments
    /// - 'vs_file' - The file path of the vertex shader
    /// - 'fs_file' - The file path of the fragment shader
    ///
    /// ## Example
    /// ```
    /// let shader = Shader::new("shaders/vs.glsl", "shaders/fs.glsl");
    /// ```
    pub fn new(vs_file: &str, fs_file: &str) -> Self {
        let mut id = 0;
        let mut vs_id = 0;
        let mut fs_id = 0;

        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(512);

        unsafe {
            info_log.set_len(512 - 1);
        } // subtract 1 to skip the trailing null character

        Self::compile_shader(
            &mut vs_id,
            vs_file,
            ShaderType::Vert,
            &mut success,
            &mut info_log,
        );
        Self::compile_shader(
            &mut fs_id,
            fs_file,
            ShaderType::Frag,
            &mut success,
            &mut info_log,
        );

        Self::create_and_link_program(&mut id, vs_id, fs_id, &mut success, &mut info_log);

        Self { id }
    }

    /// Enables a shader to the currently bound vao
    ///
    /// ## Usage
    /// A shader must be enabled before making a draw call
    ///
    /// ## Example
    /// ```
    /// shader.enable();
    /// ```
    pub fn enable(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    /// Disables an attribute in the currently bound vao.
    ///
    /// ## Usage
    /// A shader should be disabled when binding another or not in use,</br>
    /// however it is not absolutely neccessary in performance critical scenarios
    ///
    /// ## Example
    /// ```
    /// shader.disable();
    /// ```
    pub fn unbind(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    fn compile_shader(
        id: &mut u32,
        file: &str,
        ty: ShaderType,
        success: &mut i32,
        info_log: &mut Vec<u8>,
    ) {
        unsafe {
            *id = gl::CreateShader(ty.shader_type());

            if let Ok(contents) = &fs::read_to_string(file) {
                let src_c_str = CString::new(contents.as_bytes()).unwrap();

                gl::ShaderSource(*id, 1, &src_c_str.as_ptr(), ptr::null());
                gl::CompileShader(*id);

                gl::GetShaderiv(*id, gl::COMPILE_STATUS, success);

                if *success != gl::TRUE as GLint {
                    gl::GetShaderInfoLog(
                        *id,
                        512,
                        ptr::null_mut(),
                        info_log.as_mut_ptr() as *mut GLchar,
                    );

                    logger::error!(
                        "Error: {} shader compilation failed\n{}",
                        if ty == ShaderType::Vert {
                            "vertex"
                        } else {
                            "fragment"
                        },
                        str::from_utf8(&info_log).unwrap()
                    );
                }
            } else {
                logger::warn!("'/{file}' does not exist!")
            }
        }
    }

    fn create_and_link_program(
        id: &mut u32,
        vs_id: u32,
        fs_id: u32,
        success: &mut i32,
        info_log: &mut Vec<u8>,
    ) {
        unsafe {
            *id = gl::CreateProgram();

            gl::AttachShader(*id, vs_id);
            gl::AttachShader(*id, fs_id);
            gl::LinkProgram(*id);

            gl::GetProgramiv(*id, gl::LINK_STATUS, success);

            if *success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    *id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );

                logger::error!(
                    "Error: shader program failed to link\n{}",
                    str::from_utf8(&info_log).unwrap()
                )
            } else {
                logger::info!("Shader program compiled and linked successfully");
                logger::trace!("Shader program compiled and linked successfully")
            }

            gl::DeleteShader(vs_id);
            gl::DeleteShader(fs_id);
        }
    }
}
