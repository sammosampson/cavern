use crate::prelude::*;


pub struct ItemRendererItem {
    entity_id: WorldEntityId,
    dimensions: Dimensions,
    layer: u8,
    shader_program: Program,
    model_matrix: Matrix4x4,
    texture: String,
    vertex_buffer: VertexBuffer<VertexInput>,
    indices: NoIndices,
}

impl ItemRendererItem {
    pub fn new(
        screen_renderer: &ScreenRenderer,
        textures: &TextureCache,
        entity_id: WorldEntityId,
        texture: &str,
        centre_position: Vector,
        layer: u8
    ) -> Result<Self, RendererError> {
        
        let dimensions = get_sampler_texture(textures, &texture)?.dimensions();
        let centre_position = Vector::new(centre_position.x, centre_position.y);
        Ok(
            Self {
                entity_id,
                dimensions,
                layer,
                shader_program: create_render_item_shader_program(&screen_renderer)?,
                model_matrix: calculate_model_matrix(centre_position, dimensions),
                texture: texture.to_string(),
                vertex_buffer: build_unit_quad_vertex_buffer(&screen_renderer)?,
                indices: create_triangle_list_indices(),
            }
        )
    }

    pub fn layer(&self) -> u8 {
        self.layer
    }
    
    pub fn render(&self, target: &mut Frame, textures: &TextureCache) -> Result<(), RendererError> {
        println!("Rendering {:?} sized {:?}", self.entity_id, self.dimensions);
        let params = create_render_item_draw_parameters();
        
        let uniforms = uniform! {
            model: self.model_matrix.into_column_major(),
            projection: orthographic_view_matrix(0.0, SCREEN_WIDTH, 0.0, SCREEN_HEIGHT, -1.0, 1.0).into_column_major(), 
            image: get_sampler_texture(textures, &self.texture)?.sampler_wrap_clamp_minify_nearest_magnify_nearest()
        };

        self.draw_to_target(target, &params, &uniforms)?;

        Ok(())
    }

    fn draw_to_target<U:Uniforms>(&self, target: &mut Frame, params: &DrawParameters, uniforms: &U) -> Result<(), RendererError> {
        target
            .draw(
                &self.vertex_buffer, 
                &self.indices, 
                &self.shader_program, 
                uniforms, 
                &params)
            .map_err(|_|RendererError::DrawError)
    }
}

fn get_sampler_texture<'a>(textures: &'a TextureCache, texture: &'a str) -> Result<&'a SamplerTexture, RendererError> {
    let dimensions = textures
        .get(texture)
        .ok_or(RendererError::TextureLoadError)?;

    Ok(dimensions)
}

fn calculate_model_matrix(centre_position: Vector, dimensions: Dimensions) -> Matrix4x4 {
    let centre_offset = Vector::from(dimensions) * -0.5;

    translation_matrix(centre_offset) 
        * translation_matrix(centre_position) 
        * scale_matrix(dimensions)
}

fn build_unit_quad_vertex_buffer(screen_renderer: &ScreenRenderer) -> Result<VertexBuffer<VertexInput>, RendererError>{        
    Ok(
        create_vertex_buffer(&screen_renderer, &build_unit_quad())?
    )
}

fn create_triangle_list_indices() -> NoIndices {
    NoIndices(glium::index::PrimitiveType::TrianglesList)
}

fn create_render_item_shader_program(screen_renderer: &ScreenRenderer) -> Result<Program, RendererError> {    
    Ok(
        create_instance_shader_program(&screen_renderer.display)
            .map_err(|_|RendererError::FailedToCreateShaders)?
    )
}

fn create_vertex_buffer(screen_renderer: &ScreenRenderer, vertices: &Vec<VertexInput>) -> Result<VertexBuffer<VertexInput>, RendererError> {    
    Ok(
        VertexBuffer::persistent(&screen_renderer.display, vertices)
            .map_err(|_|RendererError::BufferCreationError)?
    )
}

fn create_render_item_draw_parameters<'a>() -> DrawParameters<'a> {
    DrawParameters {
        /*depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLessOrEqual,
            write: true,
            ..Default::default()
        },*/
        blend: Blend::alpha_blending(),
        .. Default::default()
    }
}