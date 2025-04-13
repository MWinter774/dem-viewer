use crate::engine::opengl;
use nalgebra_glm as glm;

pub struct PickingShaderProgram {
    shader_program: opengl::ShaderProgram,
    vertex_attrib_pointer: opengl::VertexAttributePointer,
    vertex_id_attrib_pointer: opengl::VertexAttributePointer,
    mvp_uniform_variable: std::rc::Rc<opengl::UniformVariable>,
    object_index_uniform_variable: std::rc::Rc<opengl::UniformVariable>,
}

impl PickingShaderProgram {
    pub fn new(vertices_vbo: &opengl::VBO, vertices_ids_vbo: &opengl::VBO) -> Self {
        let mut shader_program =
            opengl::ShaderProgram::new("shaders\\picking_shader.vs", "shaders\\picking_shader.fs");
        vertices_vbo.bind_as_array_buffer();
        let vertex_attrib_pointer = opengl::VertexAttributePointer::new_float(
            opengl::VertexAttributePointerConfig::default(),
        );
        let mut vertex_id_attrib_pointer_config = opengl::VertexAttributePointerConfig::default();
        vertex_id_attrib_pointer_config.index = 1;
        vertex_id_attrib_pointer_config.size = 1;
        vertex_id_attrib_pointer_config.type_ = gl::UNSIGNED_INT;
        vertices_ids_vbo.bind_as_array_buffer();
        let vertex_id_attrib_pointer =
            opengl::VertexAttributePointer::new_integer(vertex_id_attrib_pointer_config);

        let mvp_uniform_variable =
            std::rc::Rc::clone(&shader_program.get_uniform_variable("MVP").unwrap());
        let object_index_uniform_variable =
            std::rc::Rc::clone(&shader_program.get_uniform_variable("objectIndex").unwrap());
        Self {
            shader_program,
            vertex_attrib_pointer,
            vertex_id_attrib_pointer,
            mvp_uniform_variable,
            object_index_uniform_variable,
        }
    }

    pub fn use_program(&self) {
        self.shader_program.use_program();
    }

    pub fn enable_vertex_attrib_array(&self) {
        self.vertex_attrib_pointer.enable_vertex_attrib_array();
    }
    pub fn enable_vertex_id_attrib_array(&self) {
        self.vertex_id_attrib_pointer.enable_vertex_attrib_array();
    }

    pub fn set_mvp_uniform_variable(&mut self, mvp_matrix: &glm::Mat4) {
        self.shader_program
            .set_uniform_variable_matrix_4fv(&self.mvp_uniform_variable, mvp_matrix);
    }
    pub fn set_object_index_uniform_variable(&mut self, object_index: u32) {
        self.shader_program
            .set_uniform_variable_1ui(&self.object_index_uniform_variable, object_index);
    }
}
