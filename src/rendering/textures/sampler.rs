use crate::prelude::*;

pub fn create_texture(display: &Display, texture_resource: TextureResources) -> Result<SamplerTexture, TextureError> {
    SamplerTexture::new(display, get_texture_resource(texture_resource))
}

pub struct SamplerTexture {
    texture: SrgbTexture2d,
    dimensions: Dimensions  
}

impl SamplerTexture {
    fn new(display: &Display, image_data: &[u8]) -> Result<Self, TextureError> {
        let image = load_rgba_image(image_data)?;
        let dimensions = image.dimensions();
        let image = convert_to_raw_image(image, dimensions);
        let texture = create_srgb_texture(display, image)?;
        
        let texture = Self {
            texture,
            dimensions: Dimensions::from(dimensions)
        };

        Ok(texture)
    }

    pub fn sampler_wrap_clamp_minify_nearest_magnify_nearest(&self) -> Sampler<SrgbTexture2d> {
        self.sampler(
            SamplerWrapFunction::Clamp, 
            MinifySamplerFilter::Nearest, 
            MagnifySamplerFilter::Nearest)
    }

    pub fn sampler(&self, wrap: SamplerWrapFunction, minify: MinifySamplerFilter, magnify: MagnifySamplerFilter) -> Sampler<SrgbTexture2d> {
        Sampler::new(&self.texture)
            .wrap_function(wrap)
            .minify_filter(minify)
            .magnify_filter(magnify)
    }

    pub fn dimensions(&self) -> Dimensions {
        self.dimensions
    }
}

fn load_rgba_image(image_data: &[u8]) -> Result<RgbaImage, TextureError> {
    Ok(
        load(Cursor::new(image_data), ImageFormat::Png).map_err(|_|TextureError::ImageError)?.to_rgba8()
    )
}

fn convert_to_raw_image<'a>(image: ImageBuffer<Rgba<u8>, Vec<u8>>, image_dimensions: (u32, u32)) -> RawImage2d<'a, u8> {
    RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions)
}

fn create_srgb_texture(display: &Display, image: RawImage2d<u8>) -> Result<SrgbTexture2d, TextureError> {
    Ok(
        SrgbTexture2d::new(display, image).map_err(|_|TextureError::TextureCreationError)?
    )
}