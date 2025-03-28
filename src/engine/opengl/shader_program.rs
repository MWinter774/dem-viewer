use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

use crate::engine::utils;

extern crate nalgebra_glm as glm;

const SHADER_ERROR_LOG_SIZE: usize = 512;

pub struct UniformVariable(gl::types::GLint);

pub struct ShaderProgram {
    shader_program_index: gl::types::GLuint,
    uniform_variables: HashMap<String, Rc<UniformVariable>>, // Holds a mapping between uniform variable name and it's id
}

impl ShaderProgram {
    pub fn new(vertex_shader_file: &str, fragment_shader_file: &str) -> Self {
        let mut _shader_program_index: gl::types::GLuint = 0;
        let mut _vertex_shader_index: gl::types::GLuint = 0;
        let mut _fragment_shader_index: gl::types::GLuint = 0;

        // Reads the contents of the vertex and fragment shader files
        let vertex_shader_code = utils::get_contents_of_file(vertex_shader_file).unwrap();
        let fragment_shader_code = utils::get_contents_of_file(fragment_shader_file).unwrap();

        // Creates the opengl objects
        unsafe {
            _vertex_shader_index = gl::CreateShader(gl::VERTEX_SHADER);
            _fragment_shader_index = gl::CreateShader(gl::FRAGMENT_SHADER);
            _shader_program_index = gl::CreateProgram();
        }

        // Loads the code to the opengl shader objects
        unsafe {
            gl::ShaderSource(
                _vertex_shader_index,
                1 as gl::types::GLsizei,
                &utils::string_to_cstring(&vertex_shader_code).as_ptr(),
                std::ptr::null(),
            );
            gl::ShaderSource(
                _fragment_shader_index,
                1 as gl::types::GLsizei,
                &utils::string_to_cstring(&fragment_shader_code).as_ptr(),
                std::ptr::null(),
            );
        }

        // Compiles the shaders, in case of compilation error, the program panics
        ShaderProgram::compile_shader(_vertex_shader_index).unwrap();
        ShaderProgram::compile_shader(_fragment_shader_index).unwrap();

        // Link the shaders to the program. in case of error, panic
        ShaderProgram::link_program(
            _shader_program_index,
            _vertex_shader_index,
            _fragment_shader_index,
        )
        .unwrap();

        unsafe {
            gl::DetachShader(_shader_program_index, _vertex_shader_index);
            gl::DetachShader(_shader_program_index, _fragment_shader_index);
            gl::DeleteShader(_vertex_shader_index);
            gl::DeleteShader(_fragment_shader_index);
        }

        Self {
            shader_program_index: _shader_program_index,
            uniform_variables: HashMap::new(),
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::ValidateProgram(self.shader_program_index);
            gl::UseProgram(self.shader_program_index);
        }
    }

    pub fn set_uniform_variable_1f(&self, uniform_variable: &UniformVariable, val: f32) {
        unsafe {
            gl::Uniform1f(uniform_variable.0, val);
        }
    }

    pub fn set_uniform_variable_matrix_4fv(
        &self,
        uniform_variable: &UniformVariable,
        val: &glm::Mat4,
    ) {
        unsafe {
            gl::UniformMatrix4fv(uniform_variable.0, 1, gl::FALSE, val.as_ptr());
        }
    }

    /// Returns handle to the uniform variable in the uniform_variables HashMap.
    pub fn get_uniform_variable(
        &mut self,
        uniform_variable_name: &str,
    ) -> Result<Rc<UniformVariable>, String> {
        let uniform_variable_name_as_string = uniform_variable_name.to_string();

        match self.try_insert_uniform_variable(uniform_variable_name) {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        }

        Ok(Rc::clone(
            self.uniform_variables
                .get(&uniform_variable_name_as_string)
                .unwrap(),
        ))
    }

    /// If uniform_variable_name isn't already mapped to uniform_variables HashMap, then add it.
    /// Returns error if fetching uniform variable location failed.
    fn try_insert_uniform_variable(&mut self, uniform_variable_name: &str) -> Result<(), String> {
        let uniform_variable_name_as_string = uniform_variable_name.to_string();

        match self.uniform_variables.get(&uniform_variable_name_as_string) {
            Some(_) => Ok(()),
            None => match self.get_uniform_variable_location(&uniform_variable_name_as_string) {
                Ok(val) => {
                    self.uniform_variables
                        .insert(uniform_variable_name.to_string(), Rc::new(val));
                    Ok(())
                }
                Err(err) => Err(err),
            },
        }
    }

    fn get_uniform_variable_location(
        &self,
        uniform_variable_name: &String,
    ) -> Result<self::UniformVariable, String> {
        let uniform_variable_name_as_c_ptr =
            utils::string_to_cstring(uniform_variable_name).into_raw();

        let uniform_variable_location = unsafe {
            gl::GetUniformLocation(self.shader_program_index, uniform_variable_name_as_c_ptr)
        };

        match uniform_variable_location {
            -1 => Err(format!(
                "Uniform variable '{}' wasn't found!",
                uniform_variable_name
            )),
            _ => Ok(self::UniformVariable(uniform_variable_location)),
        }
    }

    /// Compiles shader (Capable of both VertexShader and FragmentShader).
    /// If fails, Result contains the error messgage.
    fn compile_shader(shader_index: gl::types::GLuint) -> Result<(), String> {
        let mut status: gl::types::GLint = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::CompileShader(shader_index);
            gl::GetShaderiv(shader_index, gl::COMPILE_STATUS, &mut status);
        }
        if status == gl::FALSE as gl::types::GLint {
            let mut log: Vec<u8> = Vec::with_capacity(SHADER_ERROR_LOG_SIZE);
            let mut log_len = 0i32;
            unsafe {
                gl::GetShaderInfoLog(
                    shader_index,
                    SHADER_ERROR_LOG_SIZE as i32,
                    &mut log_len,
                    log.as_mut_ptr().cast(),
                );
                log.set_len(log_len.try_into().unwrap());
            }
            Err(String::from_utf8_lossy(&log).into_owned())
        } else {
            Ok(())
        }
    }

    /// Links the program.
    /// If fails, Result contains the error messgage.
    fn link_program(
        shader_program_index: gl::types::GLuint,
        vertex_shader_index: gl::types::GLuint,
        fragment_shader_index: gl::types::GLuint,
    ) -> Result<(), String> {
        let mut status = gl::FALSE as gl::types::GLint;
        unsafe {
            gl::AttachShader(shader_program_index, vertex_shader_index);
            gl::AttachShader(shader_program_index, fragment_shader_index);
            gl::LinkProgram(shader_program_index);
            gl::GetProgramiv(shader_program_index, gl::LINK_STATUS, &mut status);
        }
        if status == gl::FALSE as gl::types::GLint {
            let mut log: Vec<u8> = Vec::with_capacity(SHADER_ERROR_LOG_SIZE);
            let mut log_len = 0i32;
            unsafe {
                gl::GetProgramInfoLog(
                    shader_program_index,
                    SHADER_ERROR_LOG_SIZE as i32,
                    &mut log_len,
                    log.as_mut_ptr().cast(),
                );
                log.set_len(log_len.try_into().unwrap());
            }
            Err(String::from_utf8_lossy(&log).into_owned())
        } else {
            Ok(())
        }
    }

    pub fn getAttribute(&self, name: &str) -> Result<gl::types::GLint, ()> {
        let mut res = Err(());
        unsafe {
            let val = gl::GetAttribLocation(
                self.shader_program_index,
                utils::string_to_cstring(&String::from_str(&name).unwrap()).as_ptr(),
            );
            if (val != -1) {
                res = Ok(val);
            }
        }
        res
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_program_index);
        }
    }
}
