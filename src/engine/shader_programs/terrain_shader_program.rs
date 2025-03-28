use nalgebra_glm as glm;

use crate::engine::opengl;

pub struct TerrainShaderProgram<'a> {
    shader_program: opengl::ShaderProgram,
    vertex_attrib_pointer: opengl::VertexAttributePointer<'a, f32>,
    mvp_uniform_variable: std::rc::Rc<opengl::UniformVariable>,
}

impl<'a> TerrainShaderProgram<'a> {
    pub fn new(vertices: &'a Vec<f32>) -> Self {
        let mut shader_program =
            opengl::ShaderProgram::new("shaders\\terrain_shader.vs", "shaders\\terrain_shader.fs");
        let vertex_attrib_pointer = opengl::VertexAttributePointer::new(
            vertices,
            opengl::VertexAttributePointerConfig::default(),
        );
        let mvp_uniform_variable =
            std::rc::Rc::clone(&shader_program.get_uniform_variable("MVP").unwrap());
        Self {
            shader_program,
            vertex_attrib_pointer,
            mvp_uniform_variable,
        }
    }

    pub fn use_program(&self) {
        self.shader_program.use_program();
    }

    pub fn enable_vertex_attrib_array(&self) {
        self.vertex_attrib_pointer.enable_vertex_attrib_array();
    }

    pub fn set_mvp_uniform_variable(&mut self, mvp_matrix: &glm::Mat4) {
        self.shader_program
            .set_uniform_variable_matrix_4fv(&self.mvp_uniform_variable, mvp_matrix);
    }
}
