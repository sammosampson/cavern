use crate::prelude::*;

pub fn build_unit_quad() -> Vec<VertexInput> {
    let texture_region = TextureRegion::full();
    let mut vertices: Vec::<VertexInput> = vec!();

    vertices.push(VertexInput { 
        position: [0.0, 1.0, texture_region.u_min, texture_region.v_max],
    });
    
    vertices.push(VertexInput { 
        position: [1.0, 0.0, texture_region.u_max, texture_region.v_min], 
    });
    
    vertices.push(VertexInput { 
        position: [0.0, 0.0, texture_region.u_min, texture_region.v_min], 
    });
    
    vertices.push(VertexInput { 
        position: [0.0, 1.0, texture_region.u_min, texture_region.v_max],
    });
    
    vertices.push(VertexInput { 
        position: [1.0, 1.0, texture_region.u_max, texture_region.v_max], 
    });
    
    vertices.push(VertexInput { 
        position: [1.0, 0.0, texture_region.u_max, texture_region.v_min], 
    });
    
    vertices
}