use crate::prelude::*;

fn get_vertex_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../../shaders/vertex_shader.glsl")).to_string()
}

fn get_fragment_shader_src() -> String {
    String::from_utf8_lossy(include_bytes!("../../shaders/coloured_textured_fragment_shader.glsl")).to_string()
}

fn create_shader_program(display: &Display, vertex_shader_src: &str, fragment_shader_src: &str) -> Result<Program, ProgramCreationError> {    
    Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
}

pub fn create_instance_shader_program(display: &Display) -> Result<Program, ProgramCreationError> {    
    create_shader_program(display, &get_vertex_shader_src(), &get_fragment_shader_src())
}
    
