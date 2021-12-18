use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct VertexInput {
    pub position: [f32; 4],
}

implement_vertex!(VertexInput, position);