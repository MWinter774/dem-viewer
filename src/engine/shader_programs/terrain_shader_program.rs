use nalgebra_glm as glm;

use crate::engine::opengl;

pub struct TerrainShaderProgram {
    shader_program: opengl::ShaderProgram,
    vertex_attrib_pointer: opengl::VertexAttributePointer,
    uv_vertex_attrib_pointer: opengl::VertexAttributePointer,
    mvp_uniform_variable: std::rc::Rc<opengl::UniformVariable>,
    model_matrix_uniform_variable: std::rc::Rc<opengl::UniformVariable>,
}

impl TerrainShaderProgram {
    pub fn new() -> Self {
        let mut shader_program =
            opengl::ShaderProgram::new("shaders\\terrain_shader.vs", "shaders\\terrain_shader.fs");
        let vertex_attrib_pointer =
            opengl::VertexAttributePointer::new(opengl::VertexAttributePointerConfig::default());
        let mut uv_vertex_attrib_pointer_config = opengl::VertexAttributePointerConfig::default();
        uv_vertex_attrib_pointer_config.index = 1;
        uv_vertex_attrib_pointer_config.size = 2;
        let uv_vertex_attrib_pointer =
            opengl::VertexAttributePointer::new(uv_vertex_attrib_pointer_config);
        let mvp_uniform_variable =
            std::rc::Rc::clone(&shader_program.get_uniform_variable("MVP").unwrap());
        let model_matrix_uniform_variable =
            std::rc::Rc::clone(&shader_program.get_uniform_variable("modelMatrix").unwrap());
        Self {
            shader_program,
            vertex_attrib_pointer,
            uv_vertex_attrib_pointer,
            mvp_uniform_variable,
            model_matrix_uniform_variable,
        }
    }

    pub fn use_program(&self) {
        self.shader_program.use_program();
    }

    pub fn enable_vertex_attrib_array(&self) {
        self.vertex_attrib_pointer.enable_vertex_attrib_array();
    }
    pub fn enable_uv_attrib_array(&self) {
        self.uv_vertex_attrib_pointer.enable_vertex_attrib_array();
    }

    pub fn set_mvp_uniform_variable(&mut self, mvp_matrix: &glm::Mat4) {
        self.shader_program
            .set_uniform_variable_matrix_4fv(&self.mvp_uniform_variable, mvp_matrix);
    }

    pub fn set_model_matrix_uniform_variable(&mut self, model_matrix: &glm::Mat4) {
        self.shader_program
            .set_uniform_variable_matrix_4fv(&self.model_matrix_uniform_variable, model_matrix);
    }
}
