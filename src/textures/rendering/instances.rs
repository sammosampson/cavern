use crate::prelude::*;

pub struct TextureInstanceRenderers {
    inner: Vec<TextureInstanceRenderer>
}

impl TextureInstanceRenderers {
    pub fn new(screen_renderer: &ScreenRenderer, textures: &TextureCache, to_render: Vec<TextureRenderItem>) -> Result<Self, RendererError> {    
        let mut inner = create_inner_renderers(screen_renderer, textures,to_render)?;
        sort_renderers(&mut inner);
        let renderers = Self { inner };
        Ok(renderers)
    }

    pub fn render(&self, textures: &TextureCache, target: &mut Frame) -> Result<(), RendererError> {
        for renderer in &self.inner {
            renderer.render(target, textures)?;
        }
        Ok(())
    }
}

fn create_inner_renderers(
    screen_renderer: &ScreenRenderer,
    textures: &TextureCache,
    to_render: Vec<TextureRenderItem>
) -> Result<Vec<TextureInstanceRenderer>, RendererError> {
    let to_render = to_render
        .into_iter()
        .group_by(|render_item| (render_item.texture.clone(), render_item.layer));
        
    let mut renderers = vec!();

    for ((texture, layer), grouped_items) in &to_render {
        let items: Vec<TextureRenderItem> = grouped_items.collect();
        let instances = convert_render_items_to_instance_inputs(items);
        renderers.push(TextureInstanceRenderer::new(screen_renderer, textures, &texture, instances, layer)?);
    }

    Ok(renderers)
}

fn convert_render_items_to_instance_inputs(render_items: Vec<TextureRenderItem>) -> Vec<InstanceInput> {
    render_items
        .into_iter()
        .map(|item| InstanceInput::from(item.centre_position))
        .collect()
}

fn sort_renderers(renderers: &mut Vec<TextureInstanceRenderer>) {
    renderers.sort_by(| renderer_a, renderer_b | renderer_a.layer().cmp(&renderer_b.layer()));
}

pub struct TextureInstanceRenderer {
    shader_program: Program,
    model_matrix: Matrix4x4,
    texture: String,
    vertex_buffer: VertexBuffer<VertexInput>,
    indices: NoIndices,
    layer: u8,
    instances: VertexBuffer<InstanceInput>
}

impl TextureInstanceRenderer {
    pub fn new(
        screen_renderer: &ScreenRenderer,
        textures: &TextureCache,
        texture: &str,
        instances: Vec<InstanceInput>,
        layer: u8
    ) -> Result<Self, RendererError> {
        
        let dimensions = get_sampler_texture(textures, &texture)?.dimensions();
        Ok(
            Self {
                shader_program: create_render_item_shader_program(&screen_renderer)?,
                model_matrix: translation_matrix(Vector::from(dimensions) * -0.5) * scale_matrix(dimensions),
                texture: texture.to_string(),
                vertex_buffer: build_unit_quad_vertex_buffer(&screen_renderer)?,
                indices: create_triangle_list_indices(),
                layer,
                instances: create_dynamic_vertex_buffer(&screen_renderer,&instances)?
            }
        )
    }

    pub fn layer(&self) -> u8 {
        self.layer
    }

    pub fn render(&self, target: &mut Frame, textures: &TextureCache) -> Result<(), RendererError> {
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
                (&self.vertex_buffer, self.instances.per_instance().unwrap()), 
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

fn create_dynamic_vertex_buffer(screen_renderer: &ScreenRenderer, vertices: &Vec<InstanceInput>) -> Result<VertexBuffer<InstanceInput>, RendererError> {
    Ok(
        VertexBuffer::dynamic(&screen_renderer.display, vertices)
            .map_err(|_|RendererError::BufferCreationError)?
    )
}

fn create_render_item_draw_parameters<'a>() -> DrawParameters<'a> {
    DrawParameters {
        blend: Blend::alpha_blending(),
        .. Default::default()
    }
}