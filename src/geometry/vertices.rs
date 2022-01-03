use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct VertexInput {
    pub position: [f32; 4],
}

implement_vertex!(VertexInput, position);

#[derive(Copy, Clone, Debug)]
pub struct InstanceInput {
    pub world_position: [f32; 2],
}

impl From<Vector> for InstanceInput {
    fn from(from: Vector) -> Self {
        Self {
            world_position: [from.x, from.y]
        }
    }
}

implement_vertex!(InstanceInput, world_position);