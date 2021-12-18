#[derive(Debug)]
pub struct TextureRegion {
    pub u_min: f32,
    pub u_max: f32,
    pub v_min: f32,
    pub v_max: f32,
}

impl TextureRegion {
    pub fn full() -> Self {
        Self {
            u_min: 0.0,
            u_max: 1.0,
            v_min: 0.0,
            v_max: 1.0
        }
    }
}